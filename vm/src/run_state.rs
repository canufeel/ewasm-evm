use crate::stack::EVMStack;
use u256::u256::U256;
use crate::memory::{WMemory, EVMMemory};
use alloc::boxed::Box;

pub struct RunState {
    pub stack: EVMStack<U256>,
    pub memory: Box<dyn WMemory<U256, U256>>
}

impl RunState {
    pub fn new() -> Self {
        RunState {
            stack: EVMStack::new(),
            memory: Box::new(EVMMemory::new())
        }
    }
}