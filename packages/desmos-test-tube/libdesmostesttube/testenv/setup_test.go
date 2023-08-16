package testenv

import (
	abci "github.com/cometbft/cometbft/abci/types"
	"testing"
)

func TestNewTestEnvCreation(t *testing.T) {
	NewTestEnv("desmos-1", "udsm")
}

func TestBlocksGeneration(t *testing.T) {
	env := NewTestEnv("desmos-1", "udsm")

	env.BeginNewBlock(5)

	reqEndBlock := abci.RequestEndBlock{Height: env.Ctx.BlockHeight()}
	env.App.EndBlock(reqEndBlock)
	env.App.Commit()
}
