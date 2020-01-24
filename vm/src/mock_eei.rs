
use crate::eei_common::EEI;
use u256::u256::U256;

pub struct EeiMock;

impl EeiMock {
    pub fn new () -> Self {
        EeiMock {}
    }
}

impl EEI for EeiMock {
    fn get_address(&self) -> U256 {
        U256::default()
    }
    fn finish(&self, offset: *const u8, length: usize) {}
}