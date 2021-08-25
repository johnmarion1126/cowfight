use std::io;

#[path = "cow.rs"] mod cow;

fn main() {
    println!("\n \t A wild cow appeared!\n");
    let mut enemy_cow = cow::cow::new_cow();
    cow::cow::show_cow();

    println!("What will you do?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim() {
        "fight" => enemy_cow.react_to_player("fight".to_string()),
        "pet" => enemy_cow.react_to_player("pet".to_string()),
        _ => println!("User stares at the cow and does nothing"),
    }
}
