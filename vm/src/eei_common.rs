
use u256::u256::U256;

pub trait EEI {
    fn get_address(&self) -> U256;
    fn finish(&self, offset: *const u8, length: usize);
}