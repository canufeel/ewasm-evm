#![no_std]

#![feature(core_intrinsics, lang_items, alloc_error_handler)]
extern crate alloc;

use core::slice;

mod stack;
mod vm_error;
mod interpreter;
mod run_state;
mod allocator;
mod memory;
mod opcode;
mod eei;

use interpreter::Interpreter;


pub extern "C" fn run_bytecode(
    bytecode_ptr: i32,
    bytecode_len: i32
) -> i32 {
    let bytecode =
        unsafe { slice::from_raw_parts(bytecode_ptr as *const u8, bytecode_len as usize) };
    let mut interpreter = Interpreter::new(bytecode);
    match interpreter.execute() {
        Err(_) => 0,
        Ok(_) => 1
    }
}
