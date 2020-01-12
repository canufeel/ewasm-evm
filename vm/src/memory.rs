use alloc::alloc::{Layout, alloc, realloc};
use core::{
    convert::{Into, From},
    fmt::Debug,
    mem,
    ptr,
};

pub trait WMemory<A: Debug, V: Debug>: Debug {
    fn load(&self, address: A) -> Option<V>;
    fn store(&mut self, address: A, value: V);
    fn grow(&mut self, offset: A);
    fn size(&self) -> A;
}

#[derive(Debug)]
pub struct EVMMemory<V: Debug> {
    data: *mut V,
    size: usize,
}

impl <V: Debug> EVMMemory<V> {
    pub fn new() -> EVMMemory<V> {
        EVMMemory {
            data: ptr::null_mut(),
            size: 0
        }
    }
}

impl<A: Debug + Into<usize> + From<usize>, V: Debug + Default> WMemory<A, V> for EVMMemory<V> {
    fn load(&self, address: A) -> Option<V> {
        let EVMMemory { data, size} = self;
        let size_addr: usize = address.into();
        match size_addr >= 0 && size_addr < *size {
            true => {
                let mut loaded: V = V::default();
                unsafe {
                    ptr::copy(
                        data.offset(size_addr as isize),
                        &mut loaded,
                        1
                    );
                }
                Some(loaded)
            },
            false => None
        }
    }

    fn store(&mut self, address: A, value: V) {
        let EVMMemory { data, .. } = self;
        unsafe {
            let ptr = data.offset(address.into() as isize);
            *ptr = value;
        }
    }

    fn grow(&mut self, size: A) {
        let new_size = self.size + size.into();
        let typesize = mem::size_of::<V>();
        let new_data = match self.size {
            0 => {
                let layout = Layout::from_size_align(
                    new_size * typesize,
                    typesize
                ).unwrap();
                unsafe {
                    alloc(
                        layout
                    ) as *mut V
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
                    ) as *mut V
                }
            }
        };
        self.size = new_size;
        self.data = new_data;
    }

    fn size(&self) -> A {
        self.size.into()
    }
}
