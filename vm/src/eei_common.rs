use core::any::Any;
use u256::u256::U256;

pub trait EEI {
    fn get_address(&self) -> U256;
    fn finish(&mut self, offset: *const u8, length: usize);
    fn as_any(&self) -> &dyn Any;
}