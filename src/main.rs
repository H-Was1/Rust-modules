struct Insect {
    name: String,
    capability: String,
}

struct Snake {
    info: Insect,
}

struct Grasshopper {
    info: Insect,
}
trait Move {
    fn move__to(&self, x: i32, y: i32);
}

impl Move for Insect {
    fn move__to(&self, x: i32, y: i32) {
        println!("{} {} to {},{}", self.name, self.capability, x, y);
    }
}

fn make_move(thing: impl Move) {
    thing.move__to(3, -5);
}

fn main() {
    let python = Insect {
        name: "python".to_string(),
        capability:"Slithers".to_string()
    };
    let locust = Insect {
        name: "locust".to_string(),
        capability:"Flies".to_string()
    };

    make_move(python);
    make_move(locust);
}
