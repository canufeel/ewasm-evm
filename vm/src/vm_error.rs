use alloc::{string::String};
use wasm_bindgen::JsValue;
use js_sys;

pub enum VmError {
    OutOfGas(String),
    StackUnderflow(String),
    StackOverflow(String),
    InvalidJump(String),
    InvalidOpCode(String),
    Revert(String),
    OutOfRange(String),
    Stop(String),
    InternalError(String)
}

pub type VmResult<T> = core::result::Result<T, VmError>;

impl Into<JsValue> for VmError {
    fn into(self) -> JsValue {
        match self {
            VmError::OutOfGas(msg) => js_sys::Error::new(&msg).into(),
            VmError::StackUnderflow(msg) => js_sys::Error::new(&msg).into(),
            VmError::StackOverflow(msg) => js_sys::Error::new(&msg).into(),
            VmError::InvalidJump(msg) => js_sys::Error::new(&msg).into(),
            VmError::InvalidOpCode(msg) => js_sys::Error::new(&msg).into(),
            VmError::Revert(msg) => js_sys::Error::new(&msg).into(),
            VmError::OutOfRange(msg) => js_sys::Error::new(&msg).into(),
            VmError::Stop(msg) => js_sys::Error::new(&msg).into(),
            VmError::InternalError(msg) => js_sys::Error::new(&msg).into(),
        }
    }
}
