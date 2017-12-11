use std::fmt;

use player::Player;

// Idea for a list of indices borrowed from here: https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

#[derive(Debug)]
pub struct Position {
    // NB not using proper notation as it's a faff to work out with the way I'm generating the board
    pub id:    String,
    pub piece: i8,
    pub north: Option<usize>,
    pub east:  Option<usize>,
    pub south: Option<usize>,
    pub west:  Option<usize>,
}

impl Position {
    pub fn blank(id: String) -> Position {
        return Position{ id: id, piece: 0, north: None, south: None, east: None, west: None };
    }
}

pub struct Board {
    pub positions: Vec<Position>,
    pub player1: Player,
    pub player2: Player,
}

impl Board {
    pub fn new(player1: Player, player2: Player) -> Board {

        let mut board = Board {
            positions: Vec::new(),
            player1: player1,
            player2: player2,
        };

        let (mut prev_north, mut prev_south, mut prev_east, mut prev_west) = (None, None, None, None);
        for layer in 0..3 {

            let nw = board.new_blank_position(format!("{}nw", layer));
            let ne = board.new_blank_position(format!("{}ne", layer));
            let sw = board.new_blank_position(format!("{}sw", layer));
            let se = board.new_blank_position(format!("{}se", layer));

            let north = board.add_position(Position { id: format!("{}n", layer), piece: 0, north: prev_north, south: None, east: Some(nw), west: Some(sw) });
            let east  = board.add_position(Position { id: format!("{}e", layer), piece: 0, north: Some(ne), south: Some(se), east: prev_east, west: None });
            let south = board.add_position(Position { id: format!("{}s", layer), piece: 0, north: None, south: prev_south, east: Some(sw), west: Some(se) });
            let west  = board.add_position(Position { id: format!("{}w", layer), piece: 0, north: Some(nw), south: Some(sw), east: None, west: prev_west });

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
