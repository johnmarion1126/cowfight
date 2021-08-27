pub mod player {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;

    static PLAYER_FILE: &str = "player.json";

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Player {
        pub health: i64,
        points: i64,
    }

    impl Player {
        pub fn get_player_stats(&self) {
            println!("HP: {}", self.health);
            println!("Points: {}", self.points);
        }

        pub fn gain_points(&mut self) {
            self.points += 1;
        }

        pub fn save_stats(&mut self) {
            self.health = 10; // reset health
            let serialized = serde_json::to_string(&self).expect("Failed to parse to string");
            let mut file = fs::OpenOptions::new()
                .write(true)
                .open(PLAYER_FILE)
                .expect("Failed to open file");
            write!(file, "{}", serialized);
        }
    }

    pub fn get_player() -> Player {
        let file = File::open(PLAYER_FILE).expect("Failed to open file");
        let player_stats: serde_json::Value =
            serde_json::from_reader(file).expect("Failed to read file");

        Player {
            health: player_stats
                .get("health")
                .expect("Failed to get data")
                .as_i64()
                .expect("Failed to convert"),
            points: player_stats
                .get("points")
                .expect("Failed to get data")
                .as_i64()
                .expect("Failed to convert"),
        }
    }
}
