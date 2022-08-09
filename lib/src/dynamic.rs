// When using the source_files: ["path/to/lib.rs"] auto discovery
// reloadable functions need to be public and have the #[no_mangle] attribute
#[no_mangle]
pub fn do_stuff() {
    println!("doing stuff")
}
