//generics on structs
struct point<T> {
    x:T,
    y:T,
}

struct point2<T,U>{
    x:T,
    y:U,
}

impl<T> point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

enum example<T>{
    A(T),
    B(T),
}

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn main(){
    let number_list = vec![34,50,25,100,65];
    
    let mut largest1 = largest(&number_list);
    
    let char_list = vec!['y','m','a','q'];
    
    let mut largest2 = largest(&char_list);
    println!("The largest number is {largest1}");
    
    println!("The largest number is {largest2}");
    
    let p = point {x: 5,y:10};
    
    println!("p.x = {}",p.x())
}