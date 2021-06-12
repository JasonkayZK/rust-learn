extern crate organize_tests;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(organize_tests::add_two(2), 4);
}
