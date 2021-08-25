pub mod cow {
    use rand::prelude::*;
    use rand::thread_rng;

    #[derive(Debug)]
    pub enum CowState {
        DeathStare,
        Hurt,
        Happy,
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
                    self.cow_state = CowState::Hurt;
                    show_cow(&self.cow_state);
                },
                "pet" => {
                    self.attack -= 2;
                    println!("\n \tCow liked that");
                    self.cow_state = CowState::Happy;
                    show_cow(&self.cow_state);
                } 
                _ => return,
            }
        }

        pub fn change_to_default_state(&mut self) {
            self.cow_state = CowState::DeathStare;
            show_cow(&self.cow_state);
        }

        pub fn get_cow_health(&mut self) -> i8 {
            self.health
        }

    }

    fn show_cow(state: &CowState) {
        println!("                    ^__^");

        match state {
           CowState::Hurt => println!("                    (><)\\_______"),
           CowState::Happy => println!("                    (uu)\\_______"),
           CowState::Faint => println!("                    (xx)\\_______"),
           _ => println!("                    (oo)\\_______"),
        }

        println!("                    (__)\\       )\\/\\");
        println!("                        ||----w |");
        println!("                        ||     ||");
    }
    
    pub fn new_cow() -> Cow {
        Cow {
            health: thread_rng().gen_range(5..11),
            attack: thread_rng().gen_range(2..6),
            cow_state: CowState::DeathStare,
        }
    }
}
