pub mod cow {

    enum CowState {
        Wait,
        Attack,
        TakeDamage,
        Faint,
    }

    struct Cow {
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
    }

}
