trait Park: Paint {
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
impl Paint for Car {}

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
    let nissan = Car {
        info: Vehicle_Info {
            make: "Nissan".to_string(),
            model: "Sentra".to_string(),
            year: 2020,
        },
    };
    nissan.park();
    nissan.paint("red");
    paint_Silver(&nissan);
}

fn paint_Silver<T: Paint>(obj: &T) {
    obj.paint("Silver");
}

fn paint_Silver2(obj: &impl Paint) {
    obj.paint("Silver");
}

fn paint_silver3<T>(obj: &T)
where
    T: Paint,
{
    obj.paint("Silver");
}
