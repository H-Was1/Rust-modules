use std::default;
// structs
struct Triangle {
    side_a: i32,
    side_b: i32,
    side_c: i32,
}

struct Square {
    side_a: i32,
}

// traits
trait Calculator {
    fn perimeter(&self) -> i32;
}
// method implementations & default methods
impl Square {
    // default method
    fn new(side_a: i32) -> Self {
        Self { side_a }
    }
    // Struct Method implementations
    fn print_side(&self) {
        println!("{}", self.side_a);
    }
}
// trait implementations
impl Calculator for Square {
    fn perimeter(&self) -> i32 {
        &self.side_a * 4
    }
}
// Default trait implementations
impl Default for Square {
    fn default() -> Square {
        Square { side_a: 30 }
    }
}

impl Calculator for Triangle {
    fn perimeter(&self) -> i32 {
        &self.side_a + &self.side_b + &self.side_c
    }
}

// Trait B
fn print_perimeter(shape: impl Calculator) {
    let value = shape.perimeter();
    println!("{}", value);
}

fn main() {
    let Triangle_1 = Triangle {
        side_a: 10,
        side_b: 12,
        side_c: 13,
    };
    let Square_1 = Square { side_a: 10 };

    print_perimeter(Triangle_1);
    print_perimeter(Square_1);

    let default_square = Square::default();
    let new_square = Square::new(20);
    default_square.print_side();
    print_perimeter(default_square);
    print_perimeter(new_square);
}
