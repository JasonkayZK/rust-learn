#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// Init iterator
fn init_iter() {
    let v1 = vec![1, 2, 3];
    // Lazy mode
    let v1_iter = v1.iter();

    // Until use, val been created!
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

fn direct_next() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    println!("{}", v1_iter.next() == Some(&1));
    println!("{}", v1_iter.next() == Some(&2));
    println!("{}", v1_iter.next() == Some(&3));
    println!("{}", v1_iter.next() == None);
}

fn unmut_iter() {
    let v1 = vec![
        String::from("hello0"),
        String::from("hello1"),
        String::from("hello2"),
    ];

    for x in v1.iter() {
        // Compiling err: Can't modified, default is immutable
        // x.push_str("append");
        println!("{}", x);
    }

    println!("{:?}\n", v1);
}

fn mut_iter() {
    let mut v1 = vec![
        String::from("hello0"),
        String::from("hello1"),
        String::from("hello2"),
    ];

    for x in v1.iter_mut() {
        // Value modified
        x.push_str("append");
    }

    println!("v1: {:?}\n", v1);
}

fn into_iter() {
    let v1 = vec![
        String::from("hello0"),
        String::from("hello1"),
        String::from("hello2"),
    ];
    let v1_iter = v1.into_iter();

    let mut new_v1 = Vec::new();
    for mut x in v1_iter {
        // Value modified
        x.push_str("append");
        new_v1.push(x);
    }

    // Compiling err: old value has been removed!
    // println!("old v1: {:?}\n", v1);

    println!("new v1 :{:?}\n", new_v1);
}

fn sum_demo() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("total: {}", total);

    // Compiling err: v1_iter's ownership moved into sum();
    // println!("{:?}", v1_iter);
}

fn map_demo() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{}", v2 == vec![2, 3, 4]);
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

fn map_demo2() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    println!(
        "in_my_size: {}",
        in_my_size ==
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

fn main() {
    init_iter();
    direct_next();

    unmut_iter();
    mut_iter();
    into_iter();

    sum_demo();

    map_demo();
    map_demo2();
}
