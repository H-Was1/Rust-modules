#[derive(Debug)]
enum MenuChoice {
    Mainmenu,
    Start,
    Quit,
}

fn get_choice(inp: &str) -> Result<MenuChoice, String> {
    match inp {
        "mainmenu" => Ok(MenuChoice::Mainmenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("unknown choice".to_owned()),
    }
}
fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice)
}

fn pick_choice(inp: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(inp)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    pick_choice("mainmenu");
}
