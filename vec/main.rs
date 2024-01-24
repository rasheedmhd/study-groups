// pointer to where the data is stored in the heap
// size of the data stored on the heap
// capacity ( number of items to be stored on the heap )

pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    cap: usize,
}

fn main() {
    //
}