mod front_of_house {
    pub mod hosting { // Making only this pub allows ancester to refer it not access the inner code
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }
use crate::front_of_house::hosting; // bring path in scope
pub use crate::front_of_house::hosting; // Re-exporting - making it avb for others to bring it to their scope

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
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
}

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // meal.seasonal_fruit = String::from("blueberries");
// }


use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// Bringing two vals of different types
use std::fmt; // Dont use std::fmt::Result - would not tell which result to use
use std::io;  // or we can do this aliasing

/////////////////////////

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> fmt::Result {}

fn function2() -> io::Result<()> {}


/////////////////////////
//Nested paths
/////////////////////////

use std::cmp::Ordering;
use std::io;

use std::io;
use std::io::Write;

/////////////////////////
use std::{cmp::Ordering, io};

use std::io::{self, Write};
/////////////////////////

use std::collections::*; //glob operator - brings all public items in scope

