pub(crate) enum Poll<T> {
    Ready(T),
    Pending,
}

pub(crate) trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}
