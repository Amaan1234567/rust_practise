use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveway(&self,user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else( || self.most_stocked())
    }
    
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue +=1 ,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        }else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    
    let user_pref1 = Some(ShirtColor::Red);
    let giveway1 = store.giveway(user_pref1);
    
    println!(
        "The User with preference {:?} gets {:?}",
        user_pref1, giveway1
    );
    
    let user_pref2 = None;
    let giveway2 = store.giveway(user_pref2);
    println!(
        "The User with preference {:?} get {:?}",
        user_pref2, giveway2
    );
    
    let list = vec![1,2,3];
    println!("Before defining closure: {list:?}");
    
    let only_borrows = || println!("From closure: {list:?}");
    
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
    
    let mut list2 = vec![1,2,3];
    println!("Before defining closure: {list2:?}");
    let mut borrows_mutably = || list2.push(7);
    //println!("{list2:?}");
    borrows_mutably();
    println!("After calling closure: {list2:?}");
    
    let list3 = vec![1,2,3];
    println!("Before defining closure: {list3:?}");
    thread::spawn(move || println!("From thread: {list3:?}"))
        .join()
        .unwrap();
    
    let mut list4 = [
        Rectangle {width: 10 , height: 1},
        Rectangle {width: 3 , height: 5},
        Rectangle {width: 7 , height: 12},
    ];
    
    // let mut sort_operations = vec![];
    // let value = String::from("closure called");
    // list4.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    println!("{list4:#?}");
    
    let v1: Vec<i32> = vec![1,2,3];
    let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(v2,vec![2,3,4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size:u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn filters_by_size() {
        let shoes = vec! [
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            }, 
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        
        let in_my_size = shoes_in_size(shoes,10);
        
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
                
            ]
        );
    }
}