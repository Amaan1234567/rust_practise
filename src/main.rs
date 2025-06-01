struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct color(i32, i32, i32);
struct point(i32, i32, i32);

#[derive(Debug)]
struct rectangle {
    width: u32,
    height: u32,
}

impl rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &rectangle) -> bool {
        if self.width > rect.width && self.height > rect.height {
            true
        } else {
            false
        }
    }
    
    fn square(size:u32) -> Self{
        Self{
            width:size,
            height:size
        }
    }
}
fn build_user(email: &String, username: &String) -> User {
    User {
        active: true,
        username: username.clone(),
        email: email.clone(),
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("kenobi"),
        email: String::from("something@something.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("somethingelse@somethingelse.com");

    let user2 = build_user(&user1.email, &user1.username);
    let user3 = User {
        email: String::from("newthing@newthing.com"),
        ..user1
    };
    let black = color(0, 0, 0);
    let origin = point(0, 0, 0);

    let rect1 = rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = rectangle {
        width: 20,
        height: 30,
    };

    let rect3 = rectangle {
        width: 40,
        height: 20,
    };

    println!("Area of the rectangle is {}", area(&rect1));

    println!("Area of the rectangle is {}", rect1.area());

    println!("rect1 {:#?}", rect1);

    println!("will rect2 fit in rect1?: {}", rect1.can_hold(&rect2));
    println!("will rect3 fit in rect1?: {}", rect1.can_hold(&rect3));
    
    println!("square rect1: {:#?}",rectangle::square(3));
}

fn area(rectangle: &rectangle) -> u32 {
    rectangle.width * rectangle.height
}
