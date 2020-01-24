
pub mod eei;
pub mod entrypoint;
pub mod debug;

use core::any::Any;
use u256::u256::{U256, U256bytes};
use crate::eei_common::EEI;

pub struct WasmEei;

impl WasmEei {
    pub fn new() -> Self {
        WasmEei {}
    }
}

impl EEI for WasmEei {
    fn get_address(&self) -> U256 {
        let mut bytes = U256bytes::default();
        unsafe {
            eei::ethereum_getAddress(bytes[12..].as_mut_ptr() as *const u32)
        };
        bytes.into()
    }

    fn finish(&mut self, offset: *const u8, length: usize) {
        unsafe {
            eei::ethereum_finish(offset as *const u32,length as u32)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}