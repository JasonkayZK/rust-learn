static mut REQUEST_RECV: usize = 0;

fn main() {
    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
    }
}
