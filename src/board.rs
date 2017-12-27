use std::fmt;
use std::collections::HashMap;

use position::Position;

// Idea for a list of indices borrowed from here: https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

pub struct Board {
    pub positions: Vec<Position>,
    pub ids_to_positions: HashMap<String, usize>,

}

impl Board {
    pub fn new() -> Board {

        let board = Board {
            positions: Vec::new(),
            ids_to_positions: HashMap::new(),
        };

        return Board::generate_positions(board);

    }

    fn generate_positions(mut board: Board) -> Board {
        let (mut prev_north, mut prev_south, mut prev_east, mut prev_west) = (None, None, None, None);
        for layer in 0..3 {

            let nw = board.new_blank_position(format!("{}nw", layer));
            let ne = board.new_blank_position(format!("{}ne", layer));
            let sw = board.new_blank_position(format!("{}sw", layer));
            let se = board.new_blank_position(format!("{}se", layer));

            let north = board.add_position(Position::new(format!("{}n", layer), prev_north, None, Some(nw), Some(sw)));
            let east  = board.add_position(Position::new(format!("{}e", layer),Some(ne),Some(se), prev_east, None));
            let south = board.add_position(Position::new(format!("{}s", layer), None, prev_south, Some(sw), Some(se)));
            let west  = board.add_position(Position::new(format!("{}w", layer), Some(nw), Some(sw), None, prev_west));

            board.positions[nw].east = Some(north);
            board.positions[nw].south = Some(west);
            board.positions[ne].west = Some(north);
            board.positions[ne].south = Some(east);
            board.positions[sw].north = Some(west);
            board.positions[sw].east = Some(south);
            board.positions[se].north = Some(east);
            board.positions[se].west = Some(south);

            prev_north = Some(north);
            prev_east  = Some(east);
            prev_south = Some(south);
            prev_west  = Some(west);
        }

        return board;
    }

    fn add_position(&mut self, position: Position) -> usize {
        let next_index = self.positions.len();

        self.ids_to_positions.insert(position.id.to_owned(), next_index);

        self.positions.push(position);

        return next_index;
    }

    fn new_blank_position(&mut self, id: String) -> usize {
        return self.add_position(Position::blank(id));
    }

    pub fn get_id(&self, position: Option<usize>) -> String {
        match position {
            Some(p) => self.positions[p].id.to_owned(),
            None => "_".to_string(),
        }
    }

    pub fn get_position(&self, id: &str) -> Option<&Position> {
        match self.ids_to_positions.get(id) {
            Some(index) => Some(&self.positions[index.to_owned()]),
            None => None
        }
    }

    pub fn get_mut_position(&mut self, id: String) -> &mut Position {
        let index = self.ids_to_positions.get(&id).unwrap().to_owned();
        return &mut self.positions[index];
    }

    pub fn print(&self) {
        println!("{}----------{}----------{}",
            self.get_position("0nw").unwrap().piece(),
            self.get_position("0n").unwrap().piece(),
            self.get_position("0ne").unwrap().piece());
        println!("|          |          |");
        println!("|   {}------{}------{}   |",
            self.get_position("1nw").unwrap().piece(),
            self.get_position("1n").unwrap().piece(),
            self.get_position("1ne").unwrap().piece());
        println!("|   |      |      |   |");
        println!("|   |   {}--{}--{}   |   |",
            self.get_position("2nw").unwrap().piece(),
            self.get_position("2n").unwrap().piece(),
            self.get_position("2ne").unwrap().piece());
        println!("|   |   |     |   |   |");
        println!("{}---{}---{}     {}---{}---{}",
            self.get_position("0w").unwrap().piece(),
            self.get_position("1w").unwrap().piece(),
            self.get_position("2w").unwrap().piece(),
            self.get_position("2e").unwrap().piece(),
            self.get_position("1e").unwrap().piece(),
            self.get_position("0e").unwrap().piece());
        println!("|   |   |     |   |   |");
        println!("|   |   {}--{}--{}   |   |",
            self.get_position("2sw").unwrap().piece(),
            self.get_position("2s").unwrap().piece(),
            self.get_position("2se").unwrap().piece());
        println!("|   |      |      |   |");
        println!("|   {}------{}------{}   |",
            self.get_position("1sw").unwrap().piece(),
            self.get_position("1s").unwrap().piece(),
            self.get_position("1se").unwrap().piece());
        println!("|          |          |");
        println!("{}----------{}----------{}",
            self.get_position("0sw").unwrap().piece(),
            self.get_position("0s").unwrap().piece(),
            self.get_position("0se").unwrap().piece());
    }

    pub fn is_valid_position(&self, position: &String) -> bool {
        match self.get_position(position) {
            Some(_p) => true,
            None => false
        }
    }

    pub fn is_empty_position(&self, position: &String) -> bool {
        true
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut debug_string = String::new();

        for position in &self.positions {
            debug_string += &format!("({} -> {},{},{},{}), ",
                position.id,
                self.get_id(position.north),
                self.get_id(position.east),
                self.get_id(position.south),
                self.get_id(position.west));
        }

        return write!(f, "{}", debug_string);

    }
}
