package testenv

import (
	"cosmossdk.io/simapp/params"
	"encoding/json"
	"fmt"
	"github.com/cosmos/cosmos-sdk/baseapp"
	"github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/cosmos-sdk/crypto/keys/ed25519"
	"github.com/cosmos/cosmos-sdk/server"
	"github.com/cosmos/cosmos-sdk/types/tx/signing"
	"os"
	"strings"
	"time"

	// helpers
	"github.com/cosmos/cosmos-sdk/client/tx"
	"github.com/cosmos/cosmos-sdk/types/bech32"
	authsigning "github.com/cosmos/cosmos-sdk/x/auth/signing"

	dbm "github.com/cometbft/cometbft-db"
	abci "github.com/cometbft/cometbft/abci/types"
	"github.com/cometbft/cometbft/libs/log"
	tmtypes "github.com/cometbft/cometbft/proto/tendermint/types"

	tmproto "github.com/cometbft/cometbft/proto/tendermint/types"
	"github.com/cosmos/cosmos-sdk/client/flags"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	simtestutil "github.com/cosmos/cosmos-sdk/testutil/sims"
	sdk "github.com/cosmos/cosmos-sdk/types"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	genutiltypes "github.com/cosmos/cosmos-sdk/x/genutil/types"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"

	// wasmd
	"github.com/CosmWasm/wasmd/x/wasm"
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"

	// desmos
	"github.com/desmos-labs/desmos/v5/app"
	posts "github.com/desmos-labs/desmos/v5/x/posts/types"
	profiles "github.com/desmos-labs/desmos/v5/x/profiles/types"
	reports "github.com/desmos-labs/desmos/v5/x/reports/types"
)

type TestEnv struct {
	App                *app.DesmosApp
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
	ValPrivs           []*secp256k1.PrivKey
	NodeHome           string
	ChainId            string
	StakingDenom       string
}

func NewTestEnv(chainId string, stakingDenom string) TestEnv {
	nodeHome, err := os.MkdirTemp("", ".desmos-test-tube-temp-")
	if err != nil {
		panic(err)
	}

	desmosApp, validatorPrivateKey := SetupDesmosApp(nodeHome, chainId, stakingDenom)
	paramTypesRegistry := NewParamTypeRegistry()
	ctx := desmosApp.BaseApp.NewContext(false, tmproto.Header{Height: 0, ChainID: chainId, Time: time.Now().UTC()})

	env := TestEnv{
		App:                desmosApp,
		Ctx:                ctx,
		ParamTypesRegistry: *paramTypesRegistry,
		ValPrivs: []*secp256k1.PrivKey{
			validatorPrivateKey,
		},
		NodeHome:     nodeHome,
		ChainId:      chainId,
		StakingDenom: stakingDenom,
	}

	// Allow testing unoptimized contract
	wasmtypes.MaxWasmSize = 1024 * 1024 * 1024 * 1024 * 1024

	env.SetupParamTypes()
	return env
}

func SetupDesmosApp(nodeHome string, chainId string, stakingDenom string) (*app.DesmosApp, *secp256k1.PrivKey) {
	cfg := sdk.GetConfig()
	app.SetupConfig(cfg)

	appOptions := simtestutil.AppOptionsMap{
		server.FlagTrace: true,
		flags.FlagHome:   nodeHome,
	}

	db := dbm.NewMemDB()
	appInstance := app.NewDesmosApp(
		log.NewNopLogger(),
		db,
		nil,
		true,
		appOptions,
		wasm.EnableAllProposals,
	)
	baseapp.SetChainID(chainId)(appInstance.BaseApp)

	encodingConfig := app.MakeEncodingConfig()
	genesisState := app.NewDefaultGenesisState(encodingConfig.Codec)

	// Set up Wasm genesis state
	wasmGen := wasm.GenesisState{
		Params: wasmtypes.Params{
			// Allow store code without gov
			CodeUploadAccess:             wasmtypes.AllowEverybody,
			InstantiateDefaultPermission: wasmtypes.AccessTypeEverybody,
		},
	}
	genesisState[wasm.ModuleName] = encodingConfig.Codec.MustMarshalJSON(&wasmGen)

	// Set up staking genesis state
	stakingParams := stakingtypes.DefaultParams()
	stakingParams.UnbondingTime = time.Hour * 24 * 7 * 2 // 2 weeks
	stakingGen := stakingtypes.GenesisState{
		Params: stakingParams,
	}
	genesisState[stakingtypes.ModuleName] = encodingConfig.Codec.MustMarshalJSON(&stakingGen)

	// Setup an account that will be the validator
	testUserPrivKey := secp256k1.GenPrivKey()
	testAccountAddress, err := bech32.ConvertAndEncode(cfg.GetBech32AccountAddrPrefix(), testUserPrivKey.PubKey().Address())
	requireNoErr(err)
	testAccount := authtypes.NewBaseAccount(sdk.MustAccAddressFromBech32(testAccountAddress), testUserPrivKey.PubKey(), 0, 0)

	// Setup auth module
	authGenesisState := authtypes.GenesisState{
		Params:   authtypes.DefaultParams(),
		Accounts: []*types.Any{},
	}
	anyTestAccount, err := types.NewAnyWithValue(testAccount)
	requireNoErr(err)
	authGenesisState.Accounts = append(authGenesisState.Accounts, anyTestAccount)
	genesisState[authtypes.ModuleName] = encodingConfig.Codec.MustMarshalJSON(&authGenesisState)

	// Setup the bank module so that the test account will have some founds at genesis
	bankGenesisState := banktypes.DefaultGenesisState()
	bankGenesisState.Balances = append(bankGenesisState.Balances, banktypes.Balance{
		Address: testAccount.Address,
		Coins:   sdk.NewCoins(sdk.NewCoin(stakingDenom, sdk.NewInt(200000000000000))),
	})
	genesisState[banktypes.ModuleName] = encodingConfig.Codec.MustMarshalJSON(bankGenesisState)

	// Set up the validator
	delegatedAmount := sdk.NewCoin(stakingDenom, sdk.NewInt(100000000000))
	signedTx, validatorPrivateKey, err := generateCreateValidatorTx(&encodingConfig, testUserPrivKey, chainId, delegatedAmount)
	requireNoErr(err)

	genutilGen := genutiltypes.DefaultGenesisState()
	encodedTx, err := encodingConfig.TxConfig.TxJSONEncoder()(signedTx)
	requireNoErr(err)

	genutilGen.GenTxs = append(genutilGen.GenTxs, encodedTx)
	genesisState[genutiltypes.ModuleName] = encodingConfig.Codec.MustMarshalJSON(genutilGen)

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")
	requireNoErr(err)

	concensusParams := simtestutil.DefaultConsensusParams
	concensusParams.Block = &tmtypes.BlockParams{
		MaxBytes: 22020096,
		MaxGas:   -1,
	}

	// replace sdk.DefaultDenom with the provided stakingDenom, a bit of a hack, needs improvement
	stateBytes = []byte(strings.Replace(string(stateBytes), "\"stake\"", fmt.Sprintf("\"%s\"", stakingDenom), -1))

	appInstance.InitChain(
		abci.RequestInitChain{
			Validators:      []abci.ValidatorUpdate{},
			ConsensusParams: concensusParams,
			AppStateBytes:   stateBytes,
			ChainId:         chainId,
		},
	)

	return appInstance, validatorPrivateKey
}

func (env *TestEnv) BeginNewBlock(timeIncreaseSeconds uint64) {
	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	if len(validators) >= 1 {
		validator := validators[0]
		env.beginNewBlockWithProposer(validator, timeIncreaseSeconds)
	} else {
		panic("Validator not found")
	}
}

func (env *TestEnv) GetValidatorAddresses() []string {
	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	var addresses []string
	for _, validator := range validators {
		addresses = append(addresses, validator.OperatorAddress)
	}

	return addresses
}

// beginNewBlockWithProposer begins a new block with a proposer.
func (env *TestEnv) beginNewBlockWithProposer(validator stakingtypes.Validator, timeIncreaseSeconds uint64) {

	valConsAddr, err := validator.GetConsAddr()
	requireNoErr(err)

	valAddr := valConsAddr.Bytes()

	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(timeIncreaseSeconds) * time.Second)

	header := tmtypes.Header{ChainID: env.ChainId, Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
	newCtx := env.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(env.Ctx.BlockHeight() + 1)
	env.Ctx = newCtx
	lastCommitInfo := abci.CommitInfo{
		Votes: []abci.VoteInfo{{
			Validator:       abci.Validator{Address: valAddr, Power: 1000},
			SignedLastBlock: true,
		}},
	}
	reqBeginBlock := abci.RequestBeginBlock{Header: header, LastCommitInfo: lastCommitInfo}

	env.App.BeginBlock(reqBeginBlock)
	env.Ctx = env.App.NewContext(false, reqBeginBlock.Header)
}

func (env *TestEnv) SetupParamTypes() {
	pReg := env.ParamTypesRegistry

	pReg.RegisterParamSet(&profiles.Params{})
	pReg.RegisterParamSet(&posts.Params{})
	pReg.RegisterParamSet(&reports.Params{})
}

func (env *TestEnv) FundAccount(addr sdk.AccAddress, amounts sdk.Coins) error {
	if err := env.App.BankKeeper.MintCoins(env.Ctx, minttypes.ModuleName, amounts); err != nil {
		return err
	}
	return env.App.BankKeeper.SendCoinsFromModuleToAccount(env.Ctx, minttypes.ModuleName, addr, amounts)
}

func generateCreateValidatorTx(encodingConfig *params.EncodingConfig, privKey *secp256k1.PrivKey, chainId string, delegateAmount sdk.Coin) (authsigning.Tx, *secp256k1.PrivKey, error) {
	cfg := sdk.GetConfig()
	signerAddress, err := bech32.ConvertAndEncode(cfg.GetBech32AccountAddrPrefix(), privKey.PubKey().Address())
	if err != nil {
		return nil, nil, err
	}

	// Generate validator keys
	valPrivateKey := secp256k1.GenPrivKey()
	valEdPrivateKey := ed25519.GenPrivKey()
	valPub := valEdPrivateKey.PubKey()
	// Generate the validator address
	valAddr := sdk.ValAddress(privKey.PubKey().Address())

	// Prepare the MsgCreateValidator
	msg, err := stakingtypes.NewMsgCreateValidator(
		valAddr,
		valPub,
		delegateAmount,
		stakingtypes.Description{
			Moniker: chainId,
		},
		stakingtypes.NewCommissionRates(
			sdk.NewDecWithPrec(1, 1),
			sdk.NewDecWithPrec(2, 1),
			sdk.NewDecWithPrec(1, 2),
		),
		sdk.OneInt(),
	)
	if err != nil {
		return nil, nil, err
	}

	// Prepare the transaction
	txBuilder := encodingConfig.TxConfig.NewTxBuilder()
	txBuilder.SetGasLimit(200000)
	err = txBuilder.SetMsgs(msg)
	if err != nil {
		return nil, nil, err
	}
	// Sets the tx signatures
	sigData := signing.SingleSignatureData{
		SignMode: signing.SignMode_SIGN_MODE_DIRECT,
	}
	sig := signing.SignatureV2{
		PubKey:   privKey.PubKey(),
		Data:     &sigData,
		Sequence: 0,
	}
	err = txBuilder.SetSignatures(sig)
	if err != nil {
		return nil, nil, err
	}

	// Sign the transaction
	signature, err := tx.SignWithPrivKey(
		signing.SignMode_SIGN_MODE_DIRECT,
		authsigning.SignerData{
			AccountNumber: 0,
			Sequence:      0,
			ChainID:       chainId,
			Address:       signerAddress,
			PubKey:        privKey.PubKey(),
		},
		txBuilder,
		privKey,
		encodingConfig.TxConfig,
		0,
	)
	err = txBuilder.SetSignatures(signature)
	if err != nil {
		return nil, nil, err
	}

	signedTx := txBuilder.GetTx()

	// Check that the signed transaction is valid
	err = signedTx.ValidateBasic()
	if err != nil {
		return nil, nil, err
	}

	return signedTx, valPrivateKey, nil
}

func requireNoErr(err error) {
	if err != nil {
		panic(err)
	}
}

func requireNoNil(name string, nilable any) {
	if nilable == nil {
		panic(fmt.Sprintf("%s must not be nil", name))
	}
}

func requierTrue(name string, b bool) {
	if !b {
		panic(fmt.Sprintf("%s must be true", name))
	}
}
