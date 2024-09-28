#[derive(Debug)]
enum Ticket {
    BackStage(f64, String),
    Vip(f64, String),
    GeneralAdmission(f64),
}

fn main() {
    let tickets = vec![
        Ticket::BackStage(10.0, "VIP Seat".to_string()),
        Ticket::Vip(20.0, "General Admission".to_string()),
        Ticket::GeneralAdmission(15.0),
    ];
    for ticket in &tickets {
        match ticket {
            Ticket::BackStage(price, holder) => {
                println!("Backstage Ticket Holder: {:?}, Price: {:?}", holder, price)
            }
            Ticket::Vip(price, holder) => {
                println!("VIP Ticket Holder: {:?}, Price: {:?}", holder, price)
            }
            Ticket::GeneralAdmission(price) => println!("Standard Ticket - Price: {:?}", price),
        }
    }

    for ticket in &tickets {
        match ticket {
            Ticket::BackStage(price,.. ) => println!("Back Stage Ticket - Price: {:?}", price),
            Ticket::Vip(price,.. ) => println!("VIP Ticket - Price: {:?}", price),
            Ticket::GeneralAdmission(_) => println!("Standard Ticket"),
        }
    }
}
