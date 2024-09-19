trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: &str) {
        println!("{}", color)
    }
}

struct Vehicle_Info {
    make: String,
    model: String,
    year: i32,
}

struct Car {
    info: Vehicle_Info,
}

impl Park for Car {
    fn park(&self) {
        println!("parking the {}", self.info.make);
    }
}
impl Paint for Car {
    
}

struct Truck {
    info: Vehicle_Info,
}

impl Truck {
    fn unload(&self) {
        println!("unloading {}", self.info.make);
    }
}

fn main() {
    println!("Hello, world!");
}
