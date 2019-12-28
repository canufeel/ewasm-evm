use crate::stack::EVMStack;
use crate::evm_word::evm_word::EVMWord;

pub struct RunState {
    pub stack: EVMStack<EVMWord>
}

impl RunState {
    pub fn new() -> Self {
        RunState {
            stack: EVMStack::new()
        }
    }
}