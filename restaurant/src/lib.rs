use crate::back_of_house::Appetizer::{Salad, Soup};
use crate::back_of_house::Breakfast;

mod front_of_house;
pub use crate::front_of_house::hosting::add_to_waitlist;

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                seasonal_fruit: "peaches".to_string(),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // let mut meal = back_of_house::Breakfast::summer("Rye");
    let mut meal = Breakfast::summer("Rye");
    meal.toast = "Wheat".to_string();
    println!("I'd like {} toast please", meal.toast);

    let order1 = Soup;
    let order2 = Salad;
}
