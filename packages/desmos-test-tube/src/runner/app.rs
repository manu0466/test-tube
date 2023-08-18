use cosmrs::Any;
use cosmwasm_std::{Coin, Timestamp};
use prost::Message;
use serde::de::DeserializeOwned;
use test_tube::account::SigningAccount;
use test_tube::runner::result::{RunnerExecuteResult, RunnerResult};
use test_tube::runner::Runner;
use test_tube::BaseApp;

const FEE_DENOM: &str = "udsm";
const ADDRESS_PREFIX: &str = "desmos";
const CHAIN_ID: &str = "desmos-1";
const DEFAULT_GAS_ADJUSTMENT: f64 = 1.2;

#[derive(Debug, PartialEq)]
pub struct DesmosTestApp {
    inner: BaseApp,
}

impl Default for DesmosTestApp {
    fn default() -> Self {
        DesmosTestApp::new()
    }
}

impl DesmosTestApp {
    pub fn new() -> Self {
        Self {
            inner: BaseApp::new(FEE_DENOM, CHAIN_ID, ADDRESS_PREFIX, DEFAULT_GAS_ADJUSTMENT),
        }
    }

    /// Get the current block time as a timestamp
    #[allow(dead_code)]
    pub fn get_block_timestamp(&self) -> Timestamp {
        self.inner.get_block_timestamp()
    }

    /// Get the current block time in nanoseconds
    #[allow(dead_code)]
    pub fn get_block_time_nanos(&self) -> i64 {
        self.inner.get_block_time_nanos()
    }

    /// Get the current block time in seconds
    #[allow(dead_code)]
    pub fn get_block_time_seconds(&self) -> i64 {
        self.inner.get_block_time_nanos() / 1_000_000_000i64
    }

    /// Get the current block height
    #[allow(dead_code)]
    pub fn get_block_height(&self) -> i64 {
        self.inner.get_block_height()
    }

    /// Get the first validator address
    #[allow(dead_code)]
    pub fn get_first_validator_address(&self) -> RunnerResult<String> {
        self.inner.get_first_validator_address()
    }

    /// Get the first validator signing account
    #[allow(dead_code)]
    pub fn get_first_validator_signing_account(&self) -> RunnerResult<SigningAccount> {
        self.inner.get_first_validator_signing_account()
    }

    /// Increase the time of the blockchain by the given number of seconds.
    #[allow(dead_code)]
    pub fn increase_time(&self, seconds: u64) {
        self.inner.increase_time(seconds)
    }

    /// Initialize account with initial balance of any coins.
    /// This function mints new coins and send to newly created account
    #[allow(dead_code)]
    pub fn init_account(&self, coins: &[Coin]) -> RunnerResult<SigningAccount> {
        self.inner.init_account(coins)
    }
    /// Convinience function to create multiple accounts with the same
    /// Initial coins balance
    #[allow(dead_code)]
    pub fn init_accounts(&self, coins: &[Coin], count: u64) -> RunnerResult<Vec<SigningAccount>> {
        self.inner.init_accounts(coins, count)
    }

    /// Simulate transaction execution and return gas info
    #[allow(dead_code)]
    pub fn simulate_tx<I>(
        &self,
        msgs: I,
        signer: &SigningAccount,
    ) -> RunnerResult<cosmrs::proto::cosmos::base::abci::v1beta1::GasInfo>
    where
        I: IntoIterator<Item = cosmrs::Any>,
    {
        self.inner.simulate_tx(msgs, signer)
    }

    /// Set parameter set for a given subspace.
    #[allow(dead_code)]
    pub fn set_param_set(&self, subspace: &str, pset: impl Into<Any>) -> RunnerResult<()> {
        self.inner.set_param_set(subspace, pset)
    }

    /// Get parameter set for a given subspace.
    #[allow(dead_code)]
    pub fn get_param_set<P: Message + Default>(
        &self,
        subspace: &str,
        type_url: &str,
    ) -> RunnerResult<P> {
        self.inner.get_param_set(subspace, type_url)
    }
}

impl<'a> Runner<'a> for DesmosTestApp {
    fn execute_multiple<M, R>(
        &self,
        msgs: &[(M, &str)],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        M: ::prost::Message,
        R: ::prost::Message + Default,
    {
        self.inner.execute_multiple(msgs, signer)
    }

    fn execute_multiple_raw<R>(
        &self,
        msgs: Vec<cosmrs::Any>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<R>
    where
        R: prost::Message + Default,
    {
        self.inner.execute_multiple_raw(msgs, signer)
    }

    fn query<Q, R>(&self, path: &str, q: &Q) -> RunnerResult<R>
    where
        Q: ::prost::Message,
        R: ::prost::Message + DeserializeOwned + Default,
    {
        self.inner.query(path, q)
    }
}

#[cfg(test)]
mod tests {
    use std::option::Option::None;

    use crate::runner::app::DesmosTestApp;
    use cosmwasm_std::{attr, coin, coins, Addr};
    use desmos_bindings::profiles::msg::ProfilesMsg;
    use desmos_bindings::profiles::types::{MsgDeleteProfileResponse, MsgSaveProfile};
    use desmos_bindings::reports::types::{
        QueryParamsRequest as QueryReportsParamsRequest,
        QueryParamsResponse as QueryReportsParamsResponse, StandardReason,
    };
    use test_tube::account::{Account, FeeSetting};
    use test_tube::runner::*;
    use test_tube::{ExecuteResponse, Module, Wasm};

    #[test]
    fn test_init_accounts() {
        let app = DesmosTestApp::default();
        let accounts = app
            .init_accounts(&coins(100_000_000_000, "udsm"), 3)
            .unwrap();

        assert!(accounts.get(0).is_some());
        assert!(accounts.get(1).is_some());
        assert!(accounts.get(2).is_some());
        assert!(accounts.get(3).is_none());
    }

    #[test]
    fn test_get_and_set_block_timestamp() {
        let app = DesmosTestApp::default();

        let block_time_nanos = app.get_block_time_nanos();
        let block_time_seconds = app.get_block_time_seconds();

        app.increase_time(10u64);

        assert_eq!(
            app.get_block_time_nanos(),
            block_time_nanos + 10_000_000_000
        );
        assert_eq!(app.get_block_time_seconds(), block_time_seconds + 10);
    }

    #[test]
    fn test_get_block_height() {
        let app = DesmosTestApp::default();

        assert_eq!(app.get_block_height(), 1i64);

        app.increase_time(10u64);

        assert_eq!(app.get_block_height(), 2i64);
    }

    #[test]
    fn test_execute() {
        let app = DesmosTestApp::default();

        let acc = app
            .init_account(&coins(100_000_000_000_000, "udsm"))
            .unwrap()
            .with_fee_setting(FeeSetting::Custom {
                amount: coin(2_000_000, "udsm"),
                gas_limit: 3_000_000,
            });
        let addr = acc.address();

        // Create a profile
        let msg =
            ProfilesMsg::save_profile(Some("test"), None, None, None, None, Addr::unchecked(&addr));
        let res: ExecuteResponse<MsgDeleteProfileResponse> =
            app.execute(msg, MsgSaveProfile::TYPE_URL, &acc).unwrap();

        let create_denom_attrs = &res
            .events
            .iter()
            .find(|e| e.ty == "save_profile")
            .unwrap()
            .attributes;

        assert_eq!(
            create_denom_attrs
                .iter()
                .find(|attr| attr.key.eq("profile_dtag"))
                .unwrap(),
            attr("profile_dtag", "test")
        );
        assert_eq!(
            create_denom_attrs
                .iter()
                .find(|attr| attr.key.eq("profile_creator"))
                .unwrap(),
            attr("profile_creator", &addr)
        );
    }

    #[test]
    fn test_query() {
        let app = DesmosTestApp::default();

        let standard_reasons = app
            .query::<QueryReportsParamsRequest, QueryReportsParamsResponse>(
                "/desmos.reports.v1.Query/Params",
                &QueryReportsParamsRequest {},
            )
            .unwrap()
            .params
            .unwrap()
            .standard_reasons;

        // fee is no longer set
        assert_eq!(
            standard_reasons,
            [
                StandardReason {
                    id: 1,
                    title: "Spam".to_string(),
                    description: "Signal that the reported entity is spam".to_string()
                },
                StandardReason {
                    id: 2,
                    title: "Misinformative".to_string(),
                    description: "Signal that the reported entity is misinformative".to_string()
                }
            ]
        )
    }

    #[test]
    fn test_wasm_execute_and_query() {
        use cw1_whitelist::msg::*;

        let app = DesmosTestApp::default();
        let accs = app
            .init_accounts(&[coin(1_000_000_000_000, "udsm")], 2)
            .unwrap();
        let admin = &accs[0];
        let new_admin = &accs[1];

        let wasm = Wasm::new(&app);
        let wasm_byte_code = std::fs::read("./test_artifacts/cw1_whitelist.wasm").unwrap();
        let code_id = wasm
            .store_code(&wasm_byte_code, None, admin)
            .unwrap()
            .data
            .code_id;
        assert_eq!(code_id, 1);

        // initialize admins and check if the state is correct
        let init_admins = vec![admin.address()];
        let contract_addr = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    admins: init_admins.clone(),
                    mutable: true,
                },
                Some(&admin.address()),
                None,
                &[],
                admin,
            )
            .unwrap()
            .data
            .address;
        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();
        assert_eq!(admin_list.admins, init_admins);
        assert!(admin_list.mutable);

        // update admin and check again
        let new_admins = vec![new_admin.address()];
        wasm.execute::<ExecuteMsg>(
            &contract_addr,
            &ExecuteMsg::UpdateAdmins {
                admins: new_admins.clone(),
            },
            &[],
            admin,
        )
        .unwrap();

        let admin_list = wasm
            .query::<QueryMsg, AdminListResponse>(&contract_addr, &QueryMsg::AdminList {})
            .unwrap();

        assert_eq!(admin_list.admins, new_admins);
        assert!(admin_list.mutable);
    }
}
