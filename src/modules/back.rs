pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}
impl Breakfast {
    pub fn Summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("blueberries"),
        }
    }
}