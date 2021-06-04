struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    calculate_area_tuple_demo();

    calculate_area_struct_demo();
}

fn calculate_area_tuple_demo() {
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area_tuple(&rect1),
    )
}

fn calculate_area_tuple(dimensions: &(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn calculate_area_struct_demo() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        calculate_area_struct(&rect1),
    )
}

fn calculate_area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
