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

        pub fn show_cow() {
            println!("     ^__^");
            println!("     (oo)\\_______");
            println!("     (__)\\       )\\/\\");
            println!("         ||----w |");
            println!("         ||     ||");
        }

        pub fn get_cow_stats(&self) {
            println!("Health: {}", self.health);
            println!("Attack: {}", self.attack);
            println!("State: {:?}", self.cow_state);
        }
    }

    pub fn new_cow() -> Cow {
        Cow {
            health: thread_rng().gen_range(5..11),
            attack: thread_rng().gen_range(2..6),
            cow_state: CowState::Wait,
        }
    }

}
