pub mod cow {
    use rand::prelude::*;
    use rand::thread_rng;

    #[derive(Debug)]
    enum CowState {
        Wait,
        Attack,
        TakeDamage,
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

        pub fn change_cow_state(&mut self, state: String) {
            match state.as_str() {
                "Wait" => self.cow_state = CowState::Wait,
                "Attack" => self.cow_state = CowState::Attack,
                "TakeDamage" => self.cow_state = CowState::TakeDamage,
                "Faint" => self.cow_state = CowState::Faint,
                _ => return,
            }
        }
    }

    pub fn new_cow() -> Cow {
        Cow {
            health: thread_rng().gen_range(5..11),
            attack: thread_rng().gen_range(2..6),
            cow_state: CowState::Wait,
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
