use crate::stack::EVMStack;
use u256::u256::U256;
use crate::memory::{WMemory, EVMMemory};
use alloc::{boxed::Box, vec::Vec};

pub struct RunState {
    pub stack: EVMStack<U256>,
    pub memory: Box<dyn WMemory<U256>>,
    pub bytecode: Vec<u8>,
    pub pc: usize
}

impl RunState {
    pub fn new(bytecode: &[u8]) -> Self {
        RunState {
            stack: EVMStack::new(),
            memory: Box::new(EVMMemory::new()),
            pc: 0,
            bytecode: bytecode.iter().map(|a| *a).collect()
        }
    }
}