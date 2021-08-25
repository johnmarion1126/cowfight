use std::io;

#[path = "cow.rs"] mod cow;

fn main() {
    println!("\n \t A wild cow appeared!\n");
    let mut enemy_cow = cow::cow::new_cow();
    enemy_cow.change_to_default_state();
    
    while enemy_cow.get_cow_health() > 0 {
        println!("\nChoose your action (fight/pet/run)");
        execute_player_action(&mut enemy_cow, get_input());
        enemy_cow.get_cow_stats();
    }

    println!("You won!");
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn execute_player_action(enemy_cow: &mut cow::cow::Cow, input: String) {
    match input.trim() {
        "fight" => enemy_cow.react_to_player("fight".to_string()),
        "pet" => enemy_cow.react_to_player("pet".to_string()),
        _ => println!("User stares at the cow and does nothing"),
    }
}
