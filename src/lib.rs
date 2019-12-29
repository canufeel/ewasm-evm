#![no_std]

#[macro_use]
extern crate alloc;

use wasm_bindgen::prelude::*;

mod stack;
mod vm_error;
mod opcodes;
mod run_state;
pub mod evm_word;
mod signed_evm_word;

fn main() {
}
