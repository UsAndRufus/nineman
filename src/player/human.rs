use std::io;
use itertools::Itertools;

use game::GameState;
use game::Ply;
use game::Ply::*;
use player::InputHandler;

pub struct Human {
    pub player_id: i8,
}

impl Human {
    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string().to_lowercase()
    }

    fn placement_ply(&self, piece_id: String) -> Ply {
        Placement { player_id: self.player_id, piece_id: piece_id }
    }

    fn move_ply(&self, mv: (String, String)) -> Ply {
        Move { player_id: self.player_id, mv: mv }
    }

    fn mill_ply(&self, piece_id: String) -> Ply {
        Mill { player_id: self.player_id, piece_id: piece_id }
    }
}

impl InputHandler for Human {
    fn give_new_game_state(&mut self, _game_state: GameState) {
        // Don't need to do anything
    }

    fn get_placement(&mut self, available_places: Vec<Ply>) -> Ply {
        println!("Can place at: {:?}", available_places);
        loop {
            let input = self.get_input();
            let placement = self.placement_ply(input);
            if available_places.contains(&placement) {
                break placement
            }
            println!("Invalid placement");
        }
    }

    fn get_move(&mut self, available_moves: Vec<Ply>) -> Ply {
        loop {
            println!("Available moves: {:?}", available_moves);
            match self.get_input().split(",").map(|m| m.to_string()).next_tuple() {
                Some(input) => {
                    let mv = self.move_ply(input);
                    if available_moves.contains(&mv) {
                        break mv
                    }
                    println!("Invalid move");
                }
                None => println!("Invalid move, must be in format 0n,0e"),
            }
        }
    }

    fn get_mill(&mut self, available_mills: Vec<Ply>) -> Ply {
        println!("Mill! Select piece to destroy: {:?}", available_mills);
        loop {
            let input = self.get_input();
            let mill = self.mill_ply(input);
            if available_mills.contains(&mill) {
                break mill
            }
            println!("Invalid mill");
        }
    }

    fn to_string(&self) -> String {
        "Human InputHandler".to_string()
    }

    fn set_player_id(&mut self, player_id: i8) {
        self.player_id = player_id;
    }
}
