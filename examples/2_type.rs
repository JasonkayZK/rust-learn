fn main() {
    scalar_demo();

    compound_demo();
}

fn scalar_demo() {
    // Int
    let i1: i64 = 5;
    let i2: isize = 6;
    let i3 = 98_222;
    let i4 = 0xff;
    let i5 = 0o77;
    let i6 = 0b1111_0000;
    let i7 = b'A';
    println!("i1: {}", i1);
    println!("i2: {}", i2);
    println!("i3: {}", i3);
    println!("i4: {}", i4);
    println!("i5: {}", i5);
    println!("i6: {}", i6);
    println!("i7: {}", i7);

    // Float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x: {}, y: {}", x, y);

    // Operator
    println!("5 / 2: {}", 5 / 2); // 2
    println!("5.0 / 2: {}", 5.0 / 2.0); // 2.5
    println!("5 % 2: {}", 5 % 2); // 1
    println!("-5 % 2: {}", -5 % 2); // -1

    // Bool
    let x = true;
    let y: bool = false;
    println!("x: {}, y: {}", x, y);

    // Char
    let x = '你'; // Unicode
    let y = '好';
    let z = '🐖';
    println!("x: {}, y: {}, z: {}", x, y, z);
}

fn compound_demo() {
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);

    let tup = (500, 6.4, 1); // (i32, f64, i32)
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);

    // Array
    let a = [1,2,3,4,5];
    println!("a[0]: {}", a[0]);

    // Panic: Out of bound
    // Will NOT build-pass in rust: v1.51.1
    // let index = 10;
    // let elem = a[index];
    // println!("a[10]: {}", elem);
}
