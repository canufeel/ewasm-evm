use alloc::{boxed::Box};
use core::{slice};
use crate::interpreter::Interpreter;
use super::WasmEei;
use crate::parser::OpcodeParser;
use crate::vm_error::VmError;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn runBytecode(
    bytecode_ptr: *const u8,
    bytecode_len: i32
) -> i32 {
    let bytecode =
        unsafe { slice::from_raw_parts(bytecode_ptr, bytecode_len as usize) };
    let bytecode_vec = bytecode.iter().map(|a| *a).collect();
    let eei = WasmEei::new();
    let mut interpreter = Interpreter::new(bytecode_vec, Box::new(eei));
    match interpreter.execute() {
        Err(_) => 0,
        Ok(_) => 1
    }
}

#[allow(non_snake_case)]
#[no_mangle]
extern "C" {
    fn humanizeBytecodeCaptureReturn(ret_ptr: *const u8, ret_len: usize);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn humanizeBytecode(
    bytecode_ptr: *const u8,
    bytecode_len: i32
) {
    let res = {
        let bytecode =
            unsafe { slice::from_raw_parts(bytecode_ptr, bytecode_len as usize) };
        let bytecode_vec = bytecode.iter().map(|a| *a).collect();
        let parser = OpcodeParser::new(bytecode_vec);
        parser.parse()
    };

    match res {
        Err(e) => {

        },
        Ok(mut x) => {
            let len = x.len();
            let ptr = x.as_ptr();
            unsafe { humanizeBytecodeCaptureReturn(ptr, len); }
        }
    };
}