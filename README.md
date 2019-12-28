# ewasm-evm
Implementation of Ethereum Virtual Machine compatible with EWASM API

Please note that this is a WIP so do not expect it to work.

## Rationale
1. WASM is faster then JS. This repo aims to produce code compilable to WASM which can then be used as drop-in replacement for interpreter like [ethereumjs-vm](https://github.com/ethereumjs/ethereumjs-vm).
2. Eth 2.0 design docs specify ([EEI](https://github.com/ewasm/design/blob/master/eth_interface.md)) which is Ethereum Environment Interface which as it seems would become the substitution to what is now offered by EVM itself. This project aims to provide backwards compatible interface so EVM bytecode code can be executed on top of EWASM vm.
