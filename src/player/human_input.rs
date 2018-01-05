use std::io;
use itertools::Itertools;

use player::InputHandler;

pub struct HumanInput {}

impl HumanInput {
    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string().to_lowercase()
    }
}

impl InputHandler for HumanInput {
    // TODO: loop til valid
    fn get_placement(&self) -> String {
        self.get_input()
    }

    fn get_move(&self) -> (String, String) {
        loop {
            match self.get_input().split(",").map(|m| m.to_string()).next_tuple() {
                Some(mv) => break mv,
                None => println!("Invalid move, must be in format 0n,0e"),
            }
        }
    }

    // TODO: move looping til correct mill to here
    fn get_mill(&self) -> String {
        println!("Mill! Select piece to destroy");
        self.get_input()
    }
}
