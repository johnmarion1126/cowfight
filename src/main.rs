use std::io;
use std::{thread, time};

#[path = "cow.rs"] mod cow;
#[path = "player.rs"] mod player;

fn main() {
    let mut user_player = player::player::get_player();
    user_player.get_player_stats();
    // println!("\n \t A wild cow appeared!\n");
    // let mut enemy_cow = cow::cow::new_cow();
    // enemy_cow.show_cow();
    // let DELAY = time::Duration::from_millis(500);
    // 
    // while enemy_cow.get_cow_health() > 0 {
    //     println!("\nChoose your action (fight/pet/run)");
    //     execute_player_action(&mut enemy_cow, get_input());
    //     enemy_cow.show_cow();
    //     enemy_cow.get_cow_stats();
    //     thread::sleep(DELAY);
    // }

    // println!("You won!");
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
        "run" => enemy_cow.react_to_player("run".to_string()),
        _ => println!("You stare at the cow and do nothing"),
    }
}
