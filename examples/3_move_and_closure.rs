use std::thread::spawn;

fn main() {
    let v = vec![1, 2, 3];

    // error if no "move":
    // closure may outlive the current function, but it borrows `v`, which is owned by the current function
    let handle = spawn(move || {
        println!("here is a vector {:?}", v);
    });

    // error if value "moved": value used here after move
    // drop(v);

    handle.join().unwrap();
}
