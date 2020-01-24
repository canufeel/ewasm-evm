use alloc::{string::String};
use core::cmp::PartialEq;

#[derive(Debug, PartialEq)]
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
