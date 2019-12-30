use crate::stack::EVMStack;
use u256::u256::U256;

pub struct RunState {
    pub stack: EVMStack<U256>
}

impl RunState {
    pub fn new() -> Self {
        RunState {
            stack: EVMStack::new()
        }
    }
}