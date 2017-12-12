use std::fmt;
use std::collections::HashMap;

use player::Player;
use position::Position;

// Idea for a list of indices borrowed from here: https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

pub struct Board {
    pub positions: Vec<Position>,
    pub ids_to_positions: HashMap<String, usize>,
    pub player1: Player,
    pub player2: Player,
    current_player_id: i8,

}

impl Board {
    pub fn new(player1: Player, player2: Player) -> Board {

        let board = Board {
            positions: Vec::new(),
            ids_to_positions: HashMap::new(),
            player1: player1,
            player2: player2,
            current_player_id: 1,
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

    pub fn get_position(&self, id: &str) -> &Position {
        let index = self.ids_to_positions.get(id).unwrap().to_owned();
        return &self.positions[index];
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

    pub fn current_player(&self) -> &Player {
        match self.current_player_id {
            1 => &self.player1,
            2 => &self.player2,
            _ => panic!("Invalid player: {}", self.current_player_id),
        }
    }

    pub fn make_move(&mut self) {

        let from;
        let to;
        let id;
        {
            let mv = self.get_move();
            from = mv.0;
            to   = mv.1;
            id = self.get_current_player_id();
        }

        println!("from: {}, to: {}", from, to);

        if from.is_empty() {
            self.place_piece(id, to);
        }

        self.switch_player();
    }

    fn place_piece(&mut self, player_id: i8, id: String) {
        let position = self.get_mut_position(id);
        position.place(player_id);
    }

    fn get_move(&self) -> (String, String) {
        let player = self.current_player();
        player.make_move()
    }

    fn get_current_player_id(&self) -> i8 {
        self.current_player_id
    }

    fn switch_player(&mut self) {
        match self.current_player_id {
            1 => self.current_player_id = 2,
            2 => self.current_player_id = 1,
            _ => panic!("Invalid player id: {}", self.current_player_id),
        }
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
