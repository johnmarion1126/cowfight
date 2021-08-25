pub mod cow {
    use rand::prelude::*;
    use rand::thread_rng;

    #[derive(Debug)]
    enum CowState {
        CowingAround,
        Attack,
        Faint,
    }

    pub struct Cow {
        health: i8,
        attack: i8,
        cow_state: CowState, 
    }

    impl Cow {
        pub fn get_cow_stats(&self) {
            println!("Health: {}", self.health);
            println!("Attack: {}", self.attack);
            println!("State: {:?}", self.cow_state);
        }

        pub fn react_to_player(&mut self, state: String) {
            match state.as_str() {
                "fight" => {
                    self.health -= 2;
                    println!("\n \tCow takes two damage\n");
                    show_cow();
                },
                "pet" => {
                    self.attack -= 2;
                    println!("\n \tCow liked that\n");
                    show_cow();
                } 
                _ => return,
            }
        }
    }

    pub fn new_cow() -> Cow {
        Cow {
            health: thread_rng().gen_range(5..11),
            attack: thread_rng().gen_range(2..6),
            cow_state: CowState::CowingAround,
        }
    }

    pub fn show_cow() {
        println!("                    ^__^");
        println!("                    (oo)\\_______");
        println!("                    (__)\\       )\\/\\");
        println!("                        ||----w |");
        println!("                        ||     ||");
    }
}
