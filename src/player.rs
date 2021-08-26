pub mod player {
    use std::fs;
    use std::fs::File;

    static PLAYER_FILE: &str = "player.json";

    pub struct Player {
        pub health: i64,
        pub points: i64,
        pub exp: i64,
    }

    impl Player {
        pub fn get_player_stats(&self) {
            println!("HP: {}", self.health);
            println!("Points: {}", self.points);
            println!("Exp: {}", self.exp);
        }
    }

    pub fn get_player() -> Player {
        let file = File::open(PLAYER_FILE).expect("Failed to open file");
        let player_stats: serde_json::Value = serde_json::from_reader(file).expect("Failed to read file");

        Player {
            health: player_stats.get("health").expect("Failed to get data").as_i64().expect("Failed to convert"),
            points: player_stats.get("points").expect("Failed to get data").as_i64().expect("Failed to convert"), 
            exp: player_stats.get("exp").expect("Failed to get data").as_i64().expect("Failed to convert"), 
        }
        
    }
    
}
