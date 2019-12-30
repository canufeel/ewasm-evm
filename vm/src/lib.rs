#![no_std]

#![feature(core_intrinsics, lang_items, alloc_error_handler)]
extern crate alloc;

mod stack;
mod vm_error;
mod opcodes;
mod run_state;
mod allocator;
mod memory;

use wasm_bindgen::prelude::*;

fn main() {
}
