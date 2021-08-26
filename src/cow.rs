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
                    let damage = thread_rng().gen_range(2..4);
                    self.health -= damage;
                    println!("\n \tCow takes {} damage\n", damage);
                    if self.health <= 0 {
                        self.health = 0;
                        self.cow_state = CowState::Faint;
                    }else {
                        self.cow_state = CowState::Hurt;
                    }
                },
                "pet" => {
                    println!("\n \tCow liked that");
                    self.cow_state = CowState::Happy;
                },
                "run" => {
                    println!("\n \tYou run away and cried in the shower\n");
                    self.health = 0;
                }
                _ => return,
            }
        }

        pub fn call_cow_action(&mut self, player: &mut crate::player::player::Player) {
            match self.cow_state {
                CowState::Happy => println!("\n \tThe cow loafs around"),
                CowState::Hurt => {
                    println!("\n \tThe cow attacks!");
                    let damage = thread_rng().gen_range(1..4);
                    player.health -= damage;
                    println!("\n \tYou take {} damage\n", damage);
                    self.cow_state = CowState::DeathStare;
                },
                CowState::Faint => println!("\n \tThe cow fainted!"),
                _ => println!("\n \tThe cow stares at the you"),
            }
        }

        pub fn get_cow_health(&mut self) -> i8 {
            self.health
        }

        pub fn show_cow(&self) {
            println!("                    ^__^");

            match self.cow_state {
               CowState::Hurt => println!("                    (><)\\_______"),
               CowState::Happy => println!("                    (uu)\\_______"),
               CowState::Faint => println!("                    (xx)\\_______"),
               _ => println!("                    (oo)\\_______"),
            }

            println!("                    (__)\\       )\\/\\");
            println!("                        ||----w |");
            println!("                        ||     ||");
        }
    }

    
    pub fn new_cow() -> Cow {
        Cow {
            health: thread_rng().gen_range(5..11),
            attack: thread_rng().gen_range(2..6),
            cow_state: CowState::DeathStare,
        }
    }
}
