#[path = "cow.rs"] mod cow;

fn main() {
    let enemy_cow = cow::cow::new_cow();
    enemy_cow.get_cow_stats();
}
