use core::clone::Clone;
use alloc::{collections::VecDeque, string::String};
use crate::vm_error::{VmResult, VmError};

const STACK_MAX_DEPTH: usize = 16;

pub struct EVMStack <T> {
    store: VecDeque<T>
}

impl <T: Clone> EVMStack<T> {
    pub fn new() -> Self {
        EVMStack {
            store: VecDeque::new()
        }
    }

    // pub fn length(&self) -> usize { self.store.len() }

    pub fn push(&mut self, value: T) -> VmResult<()> {
        if self.store.len() == STACK_MAX_DEPTH {
            return Err(VmError::StackOverflow(String::from("stack overflow")));
        }
        self.store.push_back(value);
        Ok(())
    }

    pub fn pop(&mut self) -> VmResult<T> {
        match self.store.pop_back() {
            None => Err(VmError::StackUnderflow(String::from("stack underflow"))),
            Some(v) => Ok(v)
        }
    }

    pub fn swap(&mut self, pos: usize) -> VmResult<()> {
        if self.store.len() <= pos || pos == 0 {
            return Err(VmError::StackUnderflow(String::from("stack underflow")))
        }

        self.store.swap(self.store.len() - 1, self.store.len() - 1 - pos);

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

#[cfg(test)]
mod tests {
    use super::*;
    use u256::u256::U256;

    #[test]
    fn swap_with_zero_pos_causes_stack_underflow() {
        let mut stack = EVMStack::new();
        stack.push(U256::default()).unwrap();
        match stack.swap(0) {
            Ok(_) => {
                assert!(false, "Should throw stack underflow error");
            },
            Err(e) => {
                assert_eq!(e, VmError::StackUnderflow(String::from("stack underflow")));
            },
        };
    }

    #[test]
    fn can_swap_two_elements() {
        let mut stack = EVMStack::new();
        let a = U256::from(256);
        let b = U256::from(128);
        stack.push(a.clone()).unwrap();
        stack.push(b).unwrap();
        match stack.swap(1) {
            Ok(_) => {
                assert_eq!(stack.pop().unwrap(), a);
            },
            Err(_) => {
                assert!(false, "Should swap 2 elements");
            },
        };
    }

    #[test]
    fn swap_with_one_element_causes_stack_underflow() {
        let mut stack = EVMStack::new();
        stack.push(U256::default()).unwrap();
        match stack.swap(1) {
            Ok(_) => {
                assert!(false, "Should throw stack underflow error");
            },
            Err(e) => {
                assert_eq!(e, VmError::StackUnderflow(String::from("stack underflow")));
            },
        };
    }
}