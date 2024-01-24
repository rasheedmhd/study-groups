// pointer to where the data is stored in the heap
// size of the data stored on the heap
// capacity ( number of items to be stored on the heap )

// using the NonNull wrapper over *mut T for have covariance
// and of course non nullable ptr
use std::ptr::NonNull;
use std::marker::PhantomData;

use std::mem;

pub struct Vec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}


// Creating an empty Vec::new() doesn't allocate data in heap
// Our ptr can't be null obviously 
// We put some garbage in there.
// cap = 0, serves as a guard that we aren't allocating anything in mem
impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(
            mem::align_of::<T>() != 0, 
            "Not handling ZST yet!"
        );
        Vec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    } 
}

fn main() {
    //
}