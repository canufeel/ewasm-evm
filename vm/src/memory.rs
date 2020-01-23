use alloc::alloc::{Layout, alloc, realloc};
use core::{
    convert::{Into, From},
    fmt::Debug,
    mem,
    ptr,
};
use u256::u256::U256bytes;

pub trait WMemory<A: Debug>: Debug {
    fn load(&self, address: A) -> Option<U256bytes>;
    fn store(&mut self, address: A, value: &[u8], size: usize);
    fn grow(&mut self, offset: usize);
    fn size(&self) -> A;
    fn address_to_memptr(&self, address: A) -> Option<*const u8>;
}

#[derive(Debug)]
pub struct EVMMemory {
    data: *mut u8,
    size: usize,
}

impl EVMMemory {
    pub fn new() -> EVMMemory {
        EVMMemory {
            data: ptr::null_mut(),
            size: 0
        }
    }
}

impl<A: Debug + Into<usize> + From<usize>> WMemory<A> for EVMMemory {
    fn load(&self, address: A) -> Option<U256bytes> {
        let EVMMemory { data, size} = self;
        let size_addr: usize = address.into();
        match size_addr >= 0 && size_addr < *size {
            true => {
                let mut loaded: U256bytes = U256bytes::default();
                unsafe {
                    ptr::copy(
                        data.offset(size_addr as isize),
                        loaded.as_mut_ptr(),
                        loaded.len()
                    );
                }
                Some(loaded)
            },
            false => None
        }
    }

    fn store(&mut self, address: A, value: &[u8], size: usize) {
        let EVMMemory { data, .. } = self;
        unsafe {
            let val_ptr = value.as_ptr();
            let ptr = data.offset(address.into() as isize);
            for i in 0..size as isize {
                *ptr.offset(i) = *val_ptr.offset(i);
            }
        }
    }

    fn grow(&mut self, size: usize) {
        let new_size = self.size + size;
        let typesize = mem::size_of::<u8>();
        let new_data = match self.size {
            0 => {
                let layout = Layout::from_size_align(
                    new_size * typesize,
                    typesize
                ).unwrap();
                unsafe {
                    alloc(
                        layout
                    ) as *mut u8
                }
            },
            _ => {
                let original_layout = Layout::from_size_align(
                    self.size * typesize,
                    typesize
                ).unwrap();
                unsafe {
                    realloc(
                        self.data as *mut u8,
                        original_layout,
                        new_size * typesize
                    ) as *mut u8
                }
            }
        };
        self.size = new_size;
        self.data = new_data;
    }

    fn size(&self) -> A {
        self.size.into()
    }

    fn address_to_memptr(&self, address: A) -> Option<*const u8> {
        let EVMMemory { data, size} = self;
        let size_addr: usize = address.into();
        match size_addr >= 0 && size_addr < *size {
            true => {
                unsafe {
                    Some(data.offset(size_addr as isize) as *const u8)
                }
            },
            false => None,
        }
    }
}
