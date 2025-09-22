package keepers

import (
	wasmtypes "github.com/CosmWasm/wasmd/x/wasm/types"
)

const (
	// DefaultHaneulInstanceCost is initially set the same as in wasmd
	DefaultHaneulInstanceCost uint64 = 60_000
	// DefaultHaneulCompileCost set to a large number for testing
	DefaultHaneulCompileCost uint64 = 3
)

// HaneulGasRegisterConfig is defaults plus a custom compile amount
func HaneulGasRegisterConfig() wasmtypes.WasmGasRegisterConfig {
	gasConfig := wasmtypes.DefaultGasRegisterConfig()
	gasConfig.InstanceCost = DefaultHaneulInstanceCost
	gasConfig.CompileCost = DefaultHaneulCompileCost

	return gasConfig
}

func NewHaneulWasmGasRegister() wasmtypes.WasmGasRegister {
	return wasmtypes.NewWasmGasRegister(HaneulGasRegisterConfig())
}
