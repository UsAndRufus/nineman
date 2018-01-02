pub use self::builder::build;

mod builder;
mod position;
mod direction;

use std::fmt;
use std::collections::HashMap;

use self::position::Position;
use self::direction::Direction;

// Idea for a list of indices borrowed from here: https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

pub struct Board {
    pub positions: Vec<Position>,
    pub ids_to_positions: HashMap<String, usize>,
    p1_mills: HashMap<(Position,Position,Position), bool>,
    p2_mills: HashMap<(Position,Position,Position), bool>,
}

impl Board {

    // move these over to builder at some point
    fn add_position(&mut self, position: Position) -> usize {
        let next_index = self.positions.len();

        self.ids_to_positions.insert(position.id.to_owned(), next_index);

        self.positions.push(position);

        return next_index;
    }

    fn new_blank_position(&mut self, id: String) -> usize {
        return self.add_position(Position::blank(id));
    }

    pub fn place_piece(&mut self, player_id: i8, piece_id: String) {
        let position = self.get_mut_position(piece_id);
        position.place(player_id);
    }

    pub fn move_piece(&mut self, player_id: i8, from_id: String, to_id: String) {
        let can_move;
        {
            let from = self.get_position(&from_id);
            let to   = self.get_position(&to_id);
            can_move = from.owned_by(player_id) && to.is_empty() && self.are_connected(from,to);
        }

        if can_move {
            self.get_mut_position(from_id).remove();
            self.get_mut_position(to_id).place(player_id);
        }
    }

    pub fn mills(&self, player_id: i8) {
        for layer in 0..3 {
            for side in Direction::iterator() {
                if let Some(mill) = self.find_mill_for_side(player_id, layer, side) {
                    println!("Mill found for {}: {:?}", player_id, mill);
                }
            }
        }

        for cross_section in Direction::iterator() {
            if let Some(mill) = self.find_mill_for_cross_section(player_id, cross_section) {
                println!("Mill found for {}: {:?}", player_id, mill);
            }
        }
    }

    fn find_mill_for_side(&self, player_id: i8, layer: i8, side: &Direction) -> Option<(&Position, &Position, &Position)> {
        match side {
            &Direction::North => self.mill(player_id, &format!("{}ne", layer), &format!("{}n", layer), &format!("{}nw", layer)),
            &Direction::East  => self.mill(player_id, &format!("{}ne", layer), &format!("{}e", layer), &format!("{}se", layer)),
            &Direction::South => self.mill(player_id, &format!("{}se", layer), &format!("{}s", layer), &format!("{}sw", layer)),
            &Direction::West  => self.mill(player_id, &format!("{}nw", layer), &format!("{}w", layer), &format!("{}sw", layer)),
        }
    }

    fn find_mill_for_cross_section(&self, player_id: i8, cross_section: &Direction) -> Option<(&Position, &Position, &Position)> {
        match cross_section {
            &Direction::North => self.mill(player_id, "0n", "1n", "2n"),
            &Direction::East  => self.mill(player_id, "0e", "1e", "2e"),
            &Direction::South => self.mill(player_id, "0s", "1s", "2s"),
            &Direction::West  => self.mill(player_id, "0w", "1w", "2w"),
        }
    }

    fn mill(&self, player_id: i8, first: &str, second: &str, third: &str) -> Option<(&Position, &Position, &Position)> {
        let mill = (self.get_position(first), self.get_position(second), self.get_position(third));
        match self.is_mill(player_id, mill) {
            true =>  Some(mill),
            false => None,
        }
    }

    fn is_mill(&self, player_id: i8, mill: (&Position, &Position, &Position)) -> bool {
        mill.0.owned_by(player_id) &&
        mill.1.owned_by(player_id) &&
        mill.2.owned_by(player_id)
    }

    pub fn get_id(&self, position: Option<usize>) -> String {
        match position {
            Some(p) => self.positions[p].id.to_owned(),
            None => "_".to_string(),
        }
    }

    fn get_position_option(&self, id: &str) -> Option<&Position> {
        match self.ids_to_positions.get(id) {
            Some(index) => Some(&self.positions[index.to_owned()]),
            None => None
        }
    }

    fn get_position(&self, id: &str) -> &Position {
        self.get_position_option(id).unwrap()
    }

    pub fn get_mut_position(&mut self, id: String) -> &mut Position {
        let index = self.ids_to_positions.get(&id).unwrap().to_owned();
        return &mut self.positions[index];
    }

    pub fn print(&self) {
        println!("{}----------{}----------{}",
            self.get_position("0nw").piece(),
            self.get_position("0n").piece(),
            self.get_position("0ne").piece());
        println!("|          |          |");
        println!("|   {}------{}------{}   |",
            self.get_position("1nw").piece(),
            self.get_position("1n").piece(),
            self.get_position("1ne").piece());
        println!("|   |      |      |   |");
        println!("|   |   {}--{}--{}   |   |",
            self.get_position("2nw").piece(),
            self.get_position("2n").piece(),
            self.get_position("2ne").piece());
        println!("|   |   |     |   |   |");
        println!("{}---{}---{}     {}---{}---{}",
            self.get_position("0w").piece(),
            self.get_position("1w").piece(),
            self.get_position("2w").piece(),
            self.get_position("2e").piece(),
            self.get_position("1e").piece(),
            self.get_position("0e").piece());
        println!("|   |   |     |   |   |");
        println!("|   |   {}--{}--{}   |   |",
            self.get_position("2sw").piece(),
            self.get_position("2s").piece(),
            self.get_position("2se").piece());
        println!("|   |      |      |   |");
        println!("|   {}------{}------{}   |",
            self.get_position("1sw").piece(),
            self.get_position("1s").piece(),
            self.get_position("1se").piece());
        println!("|          |          |");
        println!("{}----------{}----------{}",
            self.get_position("0sw").piece(),
            self.get_position("0s").piece(),
            self.get_position("0se").piece());
    }

    pub fn is_valid_position(&self, position: &String) -> bool {
        match self.get_position_option(position) {
            Some(_p) => true,
            None => false
        }
    }

    pub fn is_empty_position(&self, position: &String) -> bool {
        match self.get_position_option(position) {
            Some(p) => p.is_empty(),
            None => false
        }
    }

    fn are_connected(&self, from: &Position, to: &Position) -> bool {
        from.connected_to(self.ids_to_positions.get(&to.id))
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut debug_string = String::new();

        for position in &self.positions {
            debug_string += &format!("({} -> {},{},{},{}; c: {}), ",
                position.id,
                self.get_id(position[Direction::North]),
                self.get_id(position[Direction::East]),
                self.get_id(position[Direction::South]),
                self.get_id(position[Direction::West]),
                position.connections());
        }

        return write!(f, "{}", debug_string);

    }
}
