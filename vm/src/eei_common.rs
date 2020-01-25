use core::any::Any;
use u256::u256::U256;

pub trait EEI {
    fn get_address(&self) -> U256;
    fn finish(&mut self, offset: *const u8, length: usize);
    fn revert(&mut self, offset: *const u8, length: usize);
    fn sload(&mut self, key_offset: *const u8, result_offset: *const u8);
    fn sstore(&mut self, key_offset: *const u8, value_offset: *const u8);
    fn as_any(&self) -> &dyn Any;
}