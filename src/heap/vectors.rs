use core::mem::size_of;
use crate::mem::alloc::kalloc;

#[repr(C, packed)]
pub struct Vec<T> {
    pointer: *mut T,
    length: usize,
    capacity: usize
}

impl<T> Vec<T> {

    pub fn new<T>() -> Self {
        if size_of::<T>() != 0 {
            Vec {
                pointer: kalloc(80 * size_of::<T>()) as *mut T,
                length: 0,
                capacity: 80 * size_of::<T>()
            }
        } else {
            Vec {
                pointer: kalloc(0) as *mut T,
                length: usize::MAX,
                capacity: 0
            }
        }
    }

    pub fn push(&self, elem: T) {
        unsafe {
            self.pointer.offset(
                todo!()
            );
    }
    }
}

// slight help from chat gippity since the rust docs werent helpful
// this is to enable indexing through the vector obviously
// thx stack overflow lol
impl<T, Idx> core::ops::Index<Idx> for Vec<T> 
where 
    Idx: core::ops::Index
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Idx) -> &T {
        if index >= self.length {
            panic!("index out of bounds: the len is {} but the index is {}", self.length, index)
        } else {
            unsafe {
                &*self.pointer.offset((index * size_of::<T>()) as isize)
            }
        }
    }
}