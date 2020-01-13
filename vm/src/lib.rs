#![no_std]

#![feature(core_intrinsics, lang_items, alloc_error_handler)]
extern crate alloc;

use wasm_bindgen::prelude::*;

mod stack;
mod vm_error;
mod interpreter;
mod run_state;
mod allocator;
mod memory;
mod opcode;
mod eei;

use interpreter::Interpreter;

#[wasm_bindgen]
pub fn run_bytecode(bytecode: &[u8]) -> Result<(), JsValue> {
    let mut interpreter = Interpreter::new(bytecode);
    match interpreter.execute() {
        Err(e) => Err(e.into()),
        Ok(x) => Ok(x)
    }
}
