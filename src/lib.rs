mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub use crate::front_of_house::hosting;

mod customer {

    use crate::back_of_house;
    use crate::front_of_house;
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        crate::front_of_house::hosting::add_to_waitlist();
        front_of_house::hosting::add_to_waitlist();

        hosting::add_to_waitlist(); // because of use statement
        let mut meal = back_of_house::Breakfast::summer("rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Result::Ok(())
}

fn function2() -> io::Result<()> {
    Result::Ok(())
}

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     Result::Ok(())
// }

// fn function2() -> IoResult<()> {
//     Result::Ok(())
// }
//
// another way to use this
