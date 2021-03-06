mod builder;
mod position;
mod direction;
mod mill;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

use term_painter::Color::*;
use term_painter::Painted;
use term_painter::ToStyle;

use self::position::Position;
use self::direction::Direction;
use self::mill::Mill;
pub use self::builder::build;

use game::switch_player_id;
use game::Ply;


// Idea for a list of indices borrowed from here: https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

#[derive(Clone, Eq, PartialEq)]
pub struct Board {
    pub positions: Vec<Position>,
    pub ids_to_positions: HashMap<String, usize>,
    p1_mills: HashSet<Mill>,
    p2_mills: HashSet<Mill>,
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

    pub fn available_places(&self, player_id: i8) -> Vec<Ply> {
        self.positions.iter()
            .filter(|p| p.is_empty())
            .map(|p| Ply::Placement{ player_id: player_id, piece_id: p.id.to_owned() })
            .collect()
    }

    pub fn available_mills(&self, current_player_id: i8, opponent_id: i8) -> Vec<Ply> {
        assert_eq!(current_player_id, switch_player_id(opponent_id));

        let opponent_position_ids: HashSet<String>
            = self.positions.iter()
                .filter(|p| p.owned_by(opponent_id))
                .map(|p| p.id.to_owned())
                .collect();

        let mills;
        match opponent_id {
            1 => mills = &self.p1_mills,
            2 => mills = &self.p2_mills,
            _ => panic!("Unknown player {}", opponent_id),
        }

        let mut in_mills = HashSet::new();
        for mill in mills {
            in_mills.insert(self.get_id(Some(mill.first)));
            in_mills.insert(self.get_id(Some(mill.second)));
            in_mills.insert(self.get_id(Some(mill.third)));
        }

        let not_in_mills: Vec<String>
            = opponent_position_ids
                .difference(&in_mills)
                .map(|p| p.to_owned())
                .collect();

        // if not_in_mills.len() == 0 {
        //     opponent_position_ids.into_iter().collect()
        // } else {
        //     not_in_mills
        // }

        not_in_mills.into_iter()
            .map(|id| Ply::Mill { player_id: current_player_id, piece_id: id })
            .collect()
    }

    pub fn available_moves(&self, player_id: i8) -> Vec<Ply> {
        let owned: Vec<&Position>
            = self.positions.iter()
                .filter(|p| p.owned_by(player_id))
                .collect();

        let mut available_moves = Vec::new();
        for position in owned {
            for c in position.connections() {
                let connection = self.positions.get(*c).unwrap();
                if connection.is_empty() {
                    let mv = (position.id.to_owned(), connection.id.to_owned());
                    available_moves.push(Ply::Move { player_id: player_id, mv: mv });
                }
            }
        }

        available_moves
    }

    pub fn place_piece(&mut self, placement_ply: Ply) {
        let position = self.get_mut_position(placement_ply.piece_id());
        position.place(placement_ply.player_id());
    }

    pub fn move_piece(&mut self, move_ply: Ply) {
        let player_id = move_ply.player_id();
        let (from_id, to_id) = move_ply.mv();
        let can_move;
        {
            let from = self.get_position(&from_id);
            let to   = self.get_position(&to_id);
            can_move = from.owned_by(player_id) && to.is_empty() && self.are_connected(from,to);
        }

        if can_move {
            self.get_mut_position(from_id).remove();
            self.get_mut_position(to_id).place(player_id);
        } else {
            panic!("Invalid move from {}: ({},{})", player_id, from_id, to_id);
        }
    }

    pub fn perform_mill(&mut self, mill_ply: Ply) {
        let id = mill_ply.piece_id();
        let player_id = mill_ply.player_id();
        let opponent_id = switch_player_id(player_id);

        let available_mills = self.available_mills(player_id, opponent_id);
        let available_mills_other = self.available_mills(opponent_id, player_id);

        {
            let position = self.get_mut_position(id);
            if !position.is_empty() && !position.owned_by(player_id) {
                position.remove();
            } else {
                panic!("Invalid mill by player {}: {}; available_mills: {:?}, available_mills_other: {:?}",
                            player_id, position.id, available_mills, available_mills_other);
            }
        }

        self.validate_mills(player_id);
    }

    fn validate_mills(&mut self, from: i8) {
        let mut mills;
        match from {
            1 => mills = self.p1_mills.clone(),
            2 => mills = self.p2_mills.clone(),
            _ => panic!("Player {} found!", from),
        }
        mills.retain(|&m| self.is_mill(2, &m));
        match from {
            1 => self.p2_mills = mills,
            2 => self.p1_mills = mills,
            _ => panic!("Player {} found!", from),
        }
    }

    // TODO: consider MillFinder struct or something - or is that too Java-y?
    pub fn update_mills(&mut self, player_id: i8) -> bool {
        let mills = self.find_mills(player_id);

        let new_mills: HashSet<Mill>;
        match player_id {
            1 => {
                new_mills = mills.difference(&self.p1_mills).map(|t| *t).collect();
                //println!("found mills: {}", self.mills_string(&mills));
                //println!("p1_mills (old): {}", self.mills_string(&self.p1_mills));
                self.p1_mills = mills;
            },
            2 => {
                new_mills = mills.difference(&self.p2_mills).map(|t| *t).collect();
                //println!("found mills: {}", self.mills_string(&mills));
                //println!("p2_mills (old): {}", self.mills_string(&self.p2_mills));
                self.p2_mills = mills;
            },
            _ => {
                panic!("Unknown player_id: {}", player_id);
            }
        }


        self.can_mill(player_id, new_mills)

    }

    pub fn can_mill(&self, player_id: i8, new_mills: HashSet<Mill>) -> bool {
        match new_mills.len() {
            0 => false,
            1 => true,
            2 => true,
            _ => {
                self.print();
                // FIXME: should be panicking here as we shouldn't be able to create 3 mills in a turn!!
                println!("DANGER: Have somehow created {} mills this turn for player {}: {}",
                    new_mills.len(), player_id, self.mills_string(&new_mills));
                true
            },
        }
    }

    fn mills_string(&self, mills: &HashSet<Mill>) -> String {
        let mut mills_string = String::new();

        for mill in mills {
            mills_string.push_str(
                &format!("({},{},{})",
                    self.positions.get(mill.first).unwrap().id,
                    self.positions.get(mill.second).unwrap().id,
                    self.positions.get(mill.third).unwrap().id,
                )
            );
        }

        mills_string
    }

    fn find_mills(&self, player_id: i8) -> HashSet<Mill> {
        let mut mills = HashSet::new();
        for layer in 0..3 {
            for side in Direction::iterator() {
                if let Some(mill) = self.find_mill_for_side(player_id, layer, side) {
                    //println!("Mill found for {}: {}", player_id, self.mill_str(&mill));
                    mills.insert(mill);
                }
            }
        }

        for cross_section in Direction::iterator() {
            if let Some(mill) = self.find_mill_for_cross_section(player_id, cross_section) {
                //println!("Mill found for {}: {}", player_id, self.mill_str(&mill));
                mills.insert(mill);
            }
        }

        mills
    }

    fn find_mill_for_side(&self, player_id: i8, layer: i8, side: &Direction) -> Option<Mill> {
        match side {
            &Direction::North => self.mill(player_id, &format!("{}ne", layer), &format!("{}n", layer), &format!("{}nw", layer)),
            &Direction::East  => self.mill(player_id, &format!("{}ne", layer), &format!("{}e", layer), &format!("{}se", layer)),
            &Direction::South => self.mill(player_id, &format!("{}se", layer), &format!("{}s", layer), &format!("{}sw", layer)),
            &Direction::West  => self.mill(player_id, &format!("{}nw", layer), &format!("{}w", layer), &format!("{}sw", layer)),
        }
    }

    fn find_mill_for_cross_section(&self, player_id: i8, cross_section: &Direction) -> Option<Mill> {
        match cross_section {
            &Direction::North => self.mill(player_id, "0n", "1n", "2n"),
            &Direction::East  => self.mill(player_id, "0e", "1e", "2e"),
            &Direction::South => self.mill(player_id, "0s", "1s", "2s"),
            &Direction::West  => self.mill(player_id, "0w", "1w", "2w"),
        }
    }

    fn mill(&self, player_id: i8, first: &str, second: &str, third: &str) -> Option<Mill> {
        let mill = Mill {
            first: self.ids_to_positions[first],
            second: self.ids_to_positions[second],
            third: self.ids_to_positions[third]};
        match self.is_mill(player_id, &mill) {
            true =>  Some(mill),
            false => None,
        }
    }

    fn is_mill(&self, player_id: i8, mill: &Mill) -> bool {
        self.positions.get(mill.first).unwrap().owned_by(player_id) &&
        self.positions.get(mill.second).unwrap().owned_by(player_id) &&
        self.positions.get(mill.third).unwrap().owned_by(player_id)
    }

    fn mill_str(&self, mill: &Mill) -> String {
        format!("({}, {}, {})",
            self.positions.get(mill.first).unwrap().id,
            self.positions.get(mill.second).unwrap().id,
            self.positions.get(mill.third).unwrap().id)
    }

    pub fn get_id(&self, position: Option<usize>) -> String {
        match position {
            Some(p) => self.positions[p].id.to_owned(),
            None => "_".to_string(),
        }
    }

    fn get_position_option(&self, id: &str) -> Option<&Position> {
        match self.ids_to_positions.get(id) {
            Some(index) => self.positions.get(*index),
            None => None,
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
            self.styled_piece("0nw"),
            self.styled_piece("0n"),
            self.styled_piece("0ne"));
        println!("|          |          |");
        println!("|   {}------{}------{}   |",
            self.styled_piece("1nw"),
            self.styled_piece("1n"),
            self.styled_piece("1ne"));
        println!("|   |      |      |   |");
        println!("|   |   {}--{}--{}   |   |",
            self.styled_piece("2nw"),
            self.styled_piece("2n"),
            self.styled_piece("2ne"));
        println!("|   |   |     |   |   |");
        println!("{}---{}---{}     {}---{}---{}",
            self.styled_piece("0w"),
            self.styled_piece("1w"),
            self.styled_piece("2w"),
            self.styled_piece("2e"),
            self.styled_piece("1e"),
            self.styled_piece("0e"));
        println!("|   |   |     |   |   |");
        println!("|   |   {}--{}--{}   |   |",
            self.styled_piece("2sw"),
            self.styled_piece("2s"),
            self.styled_piece("2se"));
        println!("|   |      |      |   |");
        println!("|   {}------{}------{}   |",
            self.styled_piece("1sw"),
            self.styled_piece("1s"),
            self.styled_piece("1se"));
        println!("|          |          |");
        println!("{}----------{}----------{}",
            self.styled_piece("0sw"),
            self.styled_piece("0s"),
            self.styled_piece("0se"));
    }

    fn styled_piece(&self, pos: &str) -> Painted<String> {
        let piece = self.get_position(pos).piece();
        match piece {
            0 => White.paint(piece.to_string()),
            1 => Green.paint(piece.to_string()),
            2 => Blue.paint(piece.to_string()),
            _ => panic!("Unknown piece type {}", piece),
        }
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
                position.connections_string());
        }

        return write!(f, "{}", debug_string);

    }
}
