use alloc::{string::String};

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