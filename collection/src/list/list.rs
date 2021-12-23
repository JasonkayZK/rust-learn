use std::cell::{Ref, RefMut};
use std::error::Error;
use std::vec::IntoIter;

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

    // fn into_iter(self) -> IntoIter<T>;
}
