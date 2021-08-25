#[path = "cow.rs"] mod cow;

fn main() {
    println!("\n \t A wild cow appeared!\n");
    let mut enemy_cow = cow::cow::new_cow();
    cow::cow::show_cow();
    enemy_cow.get_cow_stats();
    enemy_cow.change_cow_state(String::from("Attack"));
    enemy_cow.get_cow_stats();
}
