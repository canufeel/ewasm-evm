use alloc::{boxed::Box};
use core::slice;
use crate::interpreter::Interpreter;
use super::WasmEei;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn runBytecode(
    bytecode_ptr: i32,
    bytecode_len: i32
) -> i32 {
    let bytecode =
        unsafe { slice::from_raw_parts(bytecode_ptr as *const u8, bytecode_len as usize) };
    let bytecode_vec = bytecode.iter().map(|a| *a).collect();
    let eei = WasmEei::new();
    let mut interpreter = Interpreter::new(bytecode_vec, Box::new(eei));
    match interpreter.execute() {
        Err(_) => 0,
        Ok(_) => 1
    }
}