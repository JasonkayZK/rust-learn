fn main() {
    // Statement
    let x = 6;

    // Expression: {}
    let y = {
        let x = 1;
        x + 2 // No `;` here!
    };

    println!("x: {}, y: {}", x, y);
}
