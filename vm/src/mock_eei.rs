use core::{ptr, any::Any};
use crate::eei_common::EEI;
use u256::u256::U256;

pub struct EeiMock {
    pub return_data_size: usize,
    pub return_data_ptr: *mut u8
}

impl EeiMock {
    pub fn new () -> Self {
        EeiMock {
            return_data_size: 0,
            return_data_ptr: ptr::null_mut()
        }
    }
}

impl EEI for EeiMock {
    fn get_address(&self) -> U256 {
        U256::default()
    }
    fn finish(&mut self, offset: *const u8, length: usize) {
        self.return_data_size = length;
        self.return_data_ptr = offset as *mut u8;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}