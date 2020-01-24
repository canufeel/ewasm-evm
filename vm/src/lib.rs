#![no_std]

#![feature(core_intrinsics, lang_items, alloc_error_handler)]
extern crate alloc;

mod stack;
mod vm_error;
mod interpreter;
mod run_state;
mod allocator;
mod memory;
mod opcode;
mod boundary;
