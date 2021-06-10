use std::fmt::Display;

#[derive(Debug)]
struct SameTypePoint<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct DiffTypePoint<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Error(E),
}

impl<T> SameTypePoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl SameTypePoint<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> DiffTypePoint<T, U> {
    fn mix_up<V, W>(self, other: DiffTypePoint<V, W>) -> DiffTypePoint<T, W> {
        DiffTypePoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    function_demo();

    same_type_struct();
    diff_type_struct();

    enum_demo();

    method_demo();
    distinct_method_demo();
    multi_generic_demo();
}

fn function_demo() {
    let num_list = vec![34, 50, 24, 100, 25];
    let res = largest(&num_list);
    println!("largest num: {}", res);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let res = largest(&char_list);
    println!("largest char: {}", res);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T
    where T: Display {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn same_type_struct() {
    let integer = SameTypePoint { x: 5, y: 10 };
    let float = SameTypePoint { x: 1.3, y: 2.2 };

    println!("int: {:?}, float: {:?}", integer, float);

    // Compile error!
    // let not_work = SameTypePoint{x:1, y:1.2};
}

fn diff_type_struct() {
    let a = DiffTypePoint { x: 5, y: 10 };
    let b = DiffTypePoint { x: 1, y: 2.2 };
    let c = DiffTypePoint { x: 'c', y: 2.1 };

    println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
}

fn enum_demo() {
    let x = MyResult::<String, String>::Ok(String::from("ok"));
    let y = MyResult::<String, String>::Error(String::from("err"));

    println!("x: {:?}, y: {:?}", x, y);
}

fn method_demo() {
    let p = SameTypePoint { x: 5, y: 15 };
    println!("p.x = {}", p.x());
}

fn distinct_method_demo() {
    let p1 = SameTypePoint { x: 1.2, y: 2.3 };
    println!("p1 distance: {}", p1.distance_from_origin());

    // P2 has no `distance_from_origin` method!
    // let p2 = SameTypePoint{x: 1, y: 2};
    // println!("p2 distance: {}", p2.distance_from_origin());
}

fn multi_generic_demo() {
    let p1 = DiffTypePoint { x: 5, y: 10.4 };
    let p2 = DiffTypePoint { x: "HELLO", y: 'c' };

    let p3 = p1.mix_up(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
