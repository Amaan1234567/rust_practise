fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age:Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("using orange as he background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    
    let (tx,rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1,2,3] {
            tx.send(val).unwrap();
        }
    });

    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a','b','c'];

    for (index,value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let point = (3,5);
    print_coordinates(&point);

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}")
    }

    println!("at the end x = {x:?}, y = {y}");

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }// range as pattern only works with numerics and char types

    let p = Point { x: 0, y: 7};

    let Point {x:a,y:b} = p;
    let Point {x,y}= p;
    assert_eq!(0,a);
    assert_eq!(7,b);

    assert_eq!(0,x);
    assert_eq!(7,y);

    match p {
        Point {x,y:0} => println!("On the x axis at {x}"),
        Point {x: 0, y} => println!("On the y axis at {y}"),
        Point {x,y} => {
            println!("On neither axis: ({x},{y})");
        }
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        },
        Message::Write(text) => {
            println!("Text message: {text}");
        },
        Message::ChangeColor(r, g, b ) => {
            println!("Change color to red {r}, green {g} , blue {b}");
        }
    }

    let msg2 = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg2 {
        Message2::ChangeColor(Color::Rgb(r, g,b )) => {
            println!("change color to red {r}, green {g}, blue {b}");
        },
        Message2::ChangeColor(Color::Hsv(h,s ,v )) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        },
        _ => ()
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match(setting_value,new_setting_value) {
        (Some(_),Some(_)) => {
            println!("Can't overwrite an existing customized value");
        },
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {setting_value:?}");

    let numbers = (2,4,8,16,32);

    match numbers {
        (first,_,third,_,fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    if let Some(_s) = &s {
        println!("found a string");
    }

    println!("{s:?}");

    let origin = Point2 { x:0, y:0, z: 0 };

    match origin {
        Point2 {x, .. } => println!("x is {x}"),
    }

    match numbers {
        (first,.., last) => {
            println!("some numbers: {first}, {last}");
        }
    }
    
    //this wont compile
    // match numbers {
    //     (..,second, ..) => {
    //         println!("some numbers: {second}");
    //     }
    // }

    let num = Some(4);

    match num {
        Some(x) if x% 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("the number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}")
    }

    println!("at the end: x = {x:?}, y = {y}");

    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello { id: id_variable @ 3..=7 ,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        },
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}

enum Message3 {
    Hello { id:i32},
}
enum Color {
    Rgb(i32,i32,i32),
    Hsv(i32,i32,i32),
}

enum Message2 {
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

enum Message {
    Quit,
    Move {x:i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

struct Point {
    x: i32,
    y: i32,
}

struct Point2 {
    x:i32,
    y:i32,
    z:i32,
}

fn print_coordinates(&(x,y): &(i32,i32)) {
    println!("Current location: ({x},{y})");
}