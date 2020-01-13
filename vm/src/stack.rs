use core::clone::Clone;
use alloc::{collections::VecDeque, string::String};
use crate::vm_error::{VmResult, VmError};

pub struct EVMStack <T> {
    store: VecDeque<T>
}

impl <T: Clone> EVMStack<T> {
    pub fn new() -> Self {
        EVMStack {
            store: VecDeque::new()
        }
    }

    pub fn length(&self) -> usize {
        self.store.len()
    }

    pub fn push(&mut self, value: T) {
        self.store.push_back(value);
    }

    pub fn pop(&mut self) -> VmResult<T> {
        match self.store.pop_back() {
            None => Err(VmError::StackUnderflow(String::from("stack underflow"))),
            Some(v) => Ok(v)
        }
    }

    pub fn pop_n(&mut self, num: usize) -> VmResult<VecDeque<T>> {
        if self.store.len() <= num {
            return Err(VmError::StackUnderflow(String::from("stack underflow")))
        }
        let idx = self.store.len() - num;
        Ok(self.store.split_off(idx))
    }

    pub fn swap(&mut self, pos: usize) -> VmResult<()> {
        if self.store.len() <= pos {
            return Err(VmError::StackUnderflow(String::from("stack underflow")))
        }

        self.store.swap(self.store.len() - 1, pos);

        Ok(())
    }

    pub fn dup(&mut self, pos: usize) -> VmResult<()> {
        if self.store.len() <= pos {
            return Err(VmError::StackUnderflow(String::from("stack underflow")))
        }

        let elem = match self.store.get(pos) {
            Some(elem) => {
                Some(elem.clone())
            },
            None => None
        };

        match elem {
            Some(elem) => {
                self.store.push_back(elem.clone());
                Ok(())
            },
            // TODO: This should not happen
            None => Err(VmError::StackUnderflow(String::from("stack underflow")))
        }
    }
}