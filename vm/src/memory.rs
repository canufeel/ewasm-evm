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
pub struct UnsafeMem<V: Debug> {
    data: *mut V,
    size: usize,
}

impl <V: Debug> UnsafeMem<V> {
    pub fn new() -> UnsafeMem<V> {
        UnsafeMem {
            data: ptr::null_mut(),
            size: 0
        }
    }
}

impl<A: Debug + Into<usize> + From<usize>, V: Debug> WMemory<A, V> for UnsafeMem<V> {
    fn load(&self, address: A) -> Option<V> {
        let UnsafeMem { data, size} = self;
        let size_addr: usize = address.into();
        match size_addr >= 0 && size_addr < *size {
            true => {
                let loaded;
                unsafe {
                    loaded = *data.offset(size_addr as isize);
                }
                Some(loaded)
            },
            false => None
        }
    }

    fn store(&mut self, address: A, value: V) {
        let UnsafeMem { data, .. } = self;
        unsafe {
            let ptr = data.offset(address.into() as isize);
            *ptr = value;
        }
    }

    fn grow(&mut self, size: A) {
        let new_size = self.size + size.into();
        let typesize = mem::size_of::<V>();
        let original_layout = Layout::from_size_align(
            self.size * typesize,
            typesize
        ).unwrap();
        let new_data = unsafe {
            let new_data = realloc(
                self.data as *mut u8,
                original_layout,
                new_size * typesize
            );
            new_data
        } as *mut V;
        self.size = new_size;
        self.data = new_data;
    }

    fn size(&self) -> A {
        self.size.into()
    }
}
