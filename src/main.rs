hot_lib_reloader::define_lib_reloader! {
    unsafe MyLibLoader {
        // Will look for "liblib.so" (Linux), "lib.dll" (Windows), ...
        lib_name: "lib",
        // Where to load the reloadable functions from,
        // relative to current file:
        source_files: ["../lib/src/dynamic.rs"]
    }
}

fn main() {
    let mut lib = MyLibLoader::new().expect("init lib loader");

    loop {
        // this reloads the lib should it have changed
        lib.update().expect("lib update");

        // This calls the reloadable function
        lib.do_stuff();

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
