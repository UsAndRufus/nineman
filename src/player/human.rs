use std::io;
use itertools::Itertools;

use game::GameState;
use player::InputHandler;

pub struct Human {}

impl Human {
    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string().to_lowercase()
    }
}

impl InputHandler for Human {
    fn give_new_game_state(&mut self, _game_state: GameState) {
        // Don't need to do anything
    }

    fn get_placement(&mut self, available_places: Vec<String>) -> String {
        println!("Can place at: {:?}", available_places);
        loop {
            let placement = self.get_input();
            if available_places.contains(&placement) {
                break placement
            }
            println!("Invalid placement");
        }
    }

    fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String) {
        loop {
            println!("Available moves: {:?}", available_moves);
            match self.get_input().split(",").map(|m| m.to_string()).next_tuple() {
                Some(mv) => {
                    if available_moves.contains(&mv) {
                        break mv
                    }
                    println!("Invalid move");
                }
                None => println!("Invalid move, must be in format 0n,0e"),
            }
        }
    }

    // TODO: move looping til correct mill to here
    fn get_mill(&mut self, available_mills: Vec<String>) -> String {
        println!("Mill! Select piece to destroy: {:?}", available_mills);
        loop {
            let mill = self.get_input();
            if available_mills.contains(&mill) {
                break mill
            }
            println!("Invalid mill");
        }
    }

    fn to_string(&self) -> String {
        "Human InputHandler".to_string()
    }

    fn set_player_id(&mut self, _player_id: i8) {
        // Don't need to do anything
    }
}
