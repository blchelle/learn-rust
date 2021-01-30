mod front_of_house {
    // Hosting and add_to_waitlist must be declared as public so
    // eat_at_restaurant can access them
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn serve_order() {}

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

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();

        // Note how the super keyword here is used to access a function in a higher scope
        super::serve_order();

        // Here, we can freely access the cook_order method because it is a sibling method
        fn cook_order() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // This doesn't compile because fix_incorrect_order is private.
    // back_of_house::fix_incorrect_order

    // We can access the public members of a struct
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // But we cannot access the private members of a struct
    // The line below won't compile because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    // All fields of a public enum are public
    // Both the lines below will compile
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("Order1 is {:?}", order1);
    println!("Order2 is {:?}", order2);
}
