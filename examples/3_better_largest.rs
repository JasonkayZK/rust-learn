fn main() {
    let num_list = vec![34, 50, 24, 100, 25];
    let res = largest(&num_list);
    println!("largest num: {}", res);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let res = largest(&char_list);
    println!("largest char: {}", res);
}

fn largest<T>(list: &[T]) -> &T where T: PartialOrd {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item
        }
    }

    largest
}
