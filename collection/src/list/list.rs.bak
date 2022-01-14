use std::cell::{Ref, RefMut};
use std::error::Error;

pub trait List<T> {
    fn new() -> Self;

    fn length(&self) -> isize;

    fn push_front(&mut self, data: T);

    fn push_back(&mut self, data: T);

    fn pop_front(&mut self) -> Option<T>;

    fn pop_back(&mut self) -> Option<T>;

    fn peek_front(&self) -> Option<Ref<T>>;

    fn peek_back(&self) -> Option<Ref<T>>;

    fn peek_front_mut(&mut self) -> Option<RefMut<T>>;

    fn peek_back_mut(&mut self) -> Option<RefMut<T>>;

    fn get_by_idx(&self, idx: isize) -> Option<Ref<T>>;

    fn get_by_idx_mut(&self, idx: isize) -> Option<RefMut<T>>;

    fn insert_by_idx(&mut self, idx: isize, data: T) -> Result<(), Box<dyn Error>>;

    fn remove_by_idx(&mut self, idx: isize) -> Result<T, Box<dyn Error>>;

    fn traverse(&self);

    // fn into_iter(self) -> IntoIter<T>;
}
