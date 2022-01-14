use std::ptr::NonNull;
use std::marker::PhantomData;

/// A Vector implementation
///  Refer to https://github.com/rust-lang/nomicon/tree/master/src/vec
pub struct Vector<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Vector<T> {}

unsafe impl<T: Sync> Sync for Vector<T> {}

impl<T> Vector<T> {

    pub fn new() -> Self {

    }

}

mod test {

    #[test]
    fn compiling_test() {}
}
