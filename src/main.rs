use rand;
use std::{
    collections::HashMap,
    io::{self, Write},
};
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("the third element is {third}"),
        None => println!("These is no third element"),
    }

    let mut v4 = vec![100, 32, 57];
    for i in &v4 {
        println!("{i}")
    }

    for i in &mut v4 {
        *i += 50;
    }

    for i in &v4 {
        println!("{i}")
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s = String::new();

    let data = "initial contents";

    let s1 = data.to_string();
    let s2 = "initial contents".to_string();

    s.push_str("bar");

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let res = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "नमस्ते";

    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in scores {
        println!("{key}: {value}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let mut vec = Vec::new();
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    for _ in 0..101 {
        let ele = rand::random_range(0..101);
        println!("ele: {ele}");
        vec.push(ele);
        let count = hashmap.entry(ele).or_insert(0);
        *count += 1;
    }
    let mut mode = -1;
    let mut max = 0;
    for (key, value) in hashmap {
        println!("{key}:{value}");
        if max < value {
            max = value;
            mode = key;
        }
    }

    let median = vec[vec.len() / 2];
    println!("Median: {median}");
    println!("Mode: {mode}");

    let original = String::from("first");
    let mut str = original.clone();
    let mut first_char = 'a';
    for char in str.chars() {
        first_char = char;
        break;
    }

    if first_char == 'a'
        || first_char == 'e'
        || first_char == 'i'
        || first_char == 'o'
        || first_char == 'u'
    {
        str.push_str("hay");
    } else {
        str = String::from(&str[1..str.len()]);
        str.push(first_char);
        str.push_str("ay");
    }

    println!("Original String: {original}");
    println!("Pig Latin: {str}");

    use std::io;

    let mut data: HashMap<String, Vec<String>> = HashMap::new();
    let mut response = String::new();
    let mut count = 0;
    while count < 5 {
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut response)
            .expect("Did not enter correct string");
        let values = Vec::from_iter(response.split(" "));
        if values[0] != "Add" || values[2] != "to" {
            println!("Please Enter in following format: Add <Person> to <Department>");
        } else {
            count += 1;
            let dept = values[3];
            let name = values[1];
            println!("{dept}:{name}");
            let vec = data.entry(String::from(values[3])).or_insert(vec![]);
            vec.push(String::from(values[1]));
            response.clear();
        }
    }
    println!("{data:?}");
    for (key, value) in &data {
        println!("Department: {key}");
        for name in value {
            println!("\t{name}");
        }
    }
}
