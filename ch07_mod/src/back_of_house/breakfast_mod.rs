pub struct Breakfast {
    toast: String,
    seasonal_fruit: String
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast { 
            toast: String::from(toast), 
            seasonal_fruit: String::from("mango") 
        }
    }
}

