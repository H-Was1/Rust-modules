// Define the Body trait (to represent different vehicle bodies)
trait Body {
    fn body_type(&self) -> &'static str; // Returns the type of body (Car, Truck, etc.)
}

// Define the Color trait (to represent different vehicle colors)
trait Color {
    fn color_name(&self) -> &'static str; // Returns the name of the color (Red, Blue, etc.)
}

// Generic structure Vehicle with Body and Color
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

// Implementation of methods for Vehicle structure
impl<B: Body, C: Color> Vehicle<B, C> {
    // Constructor to create a new Vehicle instance
    fn new(body: B, color: C) -> Self {
        Self { body, color }
    }

    // Method to describe the vehicle (body type and color)
    fn describe(&self) {
        println!(
            "This vehicle has a {} body and is {} in color.",
            self.body.body_type(),
            self.color.color_name()
        );
    }
}

// Define specific Body types that implement the Body trait

// Car struct, implementing Body trait
struct Car;

impl Body for Car {
    fn body_type(&self) -> &'static str {
        "Car"
    }
}

// Truck struct, implementing Body trait
struct Truck;

impl Body for Truck {
    fn body_type(&self) -> &'static str {
        "Truck"
    }
}

// Motorcycle struct, implementing Body trait
struct Motorcycle;

impl Body for Motorcycle {
    fn body_type(&self) -> &'static str {
        "Motorcycle"
    }
}

// Define specific Color types that implement the Color trait

// Red struct, implementing Color trait
struct Red;

impl Color for Red {
    fn color_name(&self) -> &'static str {
        "Red"
    }
}

// Blue struct, implementing Color trait
struct Blue;

impl Color for Blue {
    fn color_name(&self) -> &'static str {
        "Blue"
    }
}

// Main function to test the Vehicle struct
fn main() {
    // Create a red car and describe it
    let red_car = Vehicle::new(Car, Red);
    red_car.describe(); // Output: "This vehicle has a Car body and is Red in color."

    // Create a blue truck and describe it
    let blue_truck = Vehicle::new(Truck, Blue);
    blue_truck.describe(); // Output: "This vehicle has a Truck body and is Blue in color."

    // Create a motorcycle and describe it
    let motorcycle = Vehicle::new(Motorcycle, Red);
    motorcycle.describe(); // Output: "This vehicle has a Motorcycle body and is Red in color."
}
