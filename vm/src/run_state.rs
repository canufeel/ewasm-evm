use crate::stack::EVMStack;
use u256::u256::U256;
use crate::memory::{WMemory, EVMMemory};
use alloc::{boxed::Box, vec::Vec};
use crate::eei_common::EEI;

pub struct RunState {
    pub stack: EVMStack<U256>,
    pub memory: Box<dyn WMemory<U256>>,
    pub bytecode: Vec<u8>,
    pub eei: Box<dyn EEI>,
    pub pc: usize
}

impl RunState {
    pub fn new(bytecode: Vec<u8>, eei: Box<dyn EEI>) -> Self {
        RunState {
            stack: EVMStack::new(),
            memory: Box::new(EVMMemory::new()),
            pc: 0,
            bytecode,
            eei
        }
    }
}