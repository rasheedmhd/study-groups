// pointer to where the data is stored in the heap
// size of the data stored on the heap
// capacity ( number of items to be stored on the heap )

// using the NonNull wrapper over *mut T for have covariance
// and of course non nullable ptr
use std::ptr::NonNull;
use std::marker::PhantomData;

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

fn main() {
    //
}