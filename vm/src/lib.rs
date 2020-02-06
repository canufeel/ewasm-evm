#![no_std]

#![feature(core_intrinsics, lang_items, alloc_error_handler)]
extern crate alloc;
extern crate cfg_if;

mod stack;
mod vm_error;
mod interpreter;
mod allocator;
mod memory;
mod opcode;
pub mod eei_common;
pub mod parser;

cfg_if::cfg_if! {
       if #[cfg(target = "wasm32-unknown-unknown")] {
            mod boundary;
            use boundary as eei;
       } else {
            mod mock_eei;
            use mock_eei as eei;
       }
}

pub mod boundary;

