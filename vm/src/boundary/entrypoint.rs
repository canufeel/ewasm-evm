
use core::slice;
use crate::interpreter::Interpreter;

#[allow(non_snake_case)]
pub extern "C" fn runBytecode(
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