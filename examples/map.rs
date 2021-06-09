use std::collections::HashMap;

fn main() {
    init1();
    init2();

    ownership();

    access_val();

    traverse();

    overlap();
    or_insert();
    update();
}

fn init1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    println!("{:?}", scores);
}

fn init2() {
    let teams = vec![String::from("blue"), String::from("yellow")];
    let init_scores = vec![10, 50];

    // Can infer type
    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();
    println!("{:?}", scores);
}

fn ownership() {
    let name = String::from("color");
    let val = String::from("blue");

    let mut map = HashMap::new();
    map.insert(name, val);
    println!("map: {:?}", map);

    // Invalid: name & val do not exist!
    // println!("name: {}, val: {}", name, val);
}

fn access_val() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    println!("blue score: {:?}", scores.get("blue"));
}

fn traverse() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    for (key, val) in &scores {
        println!("k: {}, v: {}", key, val);
    }
}

fn overlap() {
    let mut map = HashMap::new();

    map.insert(String::from("blue"), 10);
    map.insert(String::from("blue"), 25);

    println!("{:?}", map);
}

fn or_insert(){
    let mut map = HashMap::new();

    map.insert(String::from("blue"), 10);

    map.entry(String::from("yellow")).or_insert(50);
    map.entry(String::from("blue")).or_insert(50);
    println!("{:?}", map);
}

fn update() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
