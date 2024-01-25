// pointer to where the data is stored in the heap
// size of the data stored on the heap
// capacity ( number of items to be stored on the heap )

// using the NonNull wrapper over *mut T for have covariance
// and of course non nullable ptr
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::alloc::{self, Layout}

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

    fn grow(&mut self) {
        let (new_cap, new_length) if cap.len == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        }

        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe {
                alloc::alloc(new_layout)
            }
        }
    } 
}

fn main() {
    //
}