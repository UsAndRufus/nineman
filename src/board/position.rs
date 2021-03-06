use std::ops::Index;
use std::fmt;

use board::Direction;
use board::Direction::*;

#[derive(Hash, Clone)]
pub struct Position {
    // NB not using proper notation as it's a faff to work out with the way I'm generating the board
    pub id:    String,
    piece:     i8,
    // TODO: should this be a map?
    north: Option<usize>,
    east:  Option<usize>,
    south: Option<usize>,
    west:  Option<usize>,
    connections: Vec<usize>
}

impl Position {
    // TODO: consider PositionFactory
    pub fn new(id: String, north: Option<usize>, east: Option<usize>, south: Option<usize>,
               west:  Option<usize>) -> Position {
        let connections = Vec::with_capacity(4);
        let mut position = Position { id: id, piece: 0, north: north, south: south, east: east,
            west: west, connections: connections };
        position.add_connection(north);
        position.add_connection(east);
        position.add_connection(south);
        position.add_connection(west);
        position
    }

    pub fn blank(id: String) -> Position {
        return Position::new(id, None, None, None, None);
    }

    fn add_connection(&mut self, connection: Option<usize>) {
        if let Some(p) = connection {
            self.connections.push(p);
        }
    }

    pub fn add_neighbour(&mut self, direction: Direction, position: Option<usize>) {
        match direction {
            North => {
                self.north = position;
                self.add_connection(position);
            },
            East => {
                self.east = position;
                self.add_connection(position);
            },
            South => {
                self.south = position;
                self.add_connection(position);
            },
            West => {
                self.west = position;
                self.add_connection(position);
            },
        }
    }

    pub fn place(&mut self, player_id: i8) {
        match self.piece {
            0 => self.piece = player_id,
            _ => panic!("Position already has piece belonging to Player {}", self.piece)
        }
    }

    pub fn remove(&mut self) {
        self.piece = 0;
    }

    pub fn piece(&self) -> i8 {
        self.piece
    }

    pub fn is_empty(&self) -> bool {
        self.piece == 0
    }

    pub fn owned_by(&self, player_id: i8) -> bool {
        self.piece == player_id
    }

    pub fn connected_to(&self, other_option: Option<&usize>) -> bool {
        if let Some(other) = other_option {
            self.connections.contains(other)
        } else {
            false
        }
    }

    pub fn connections(&self) -> &Vec<usize> {
        &self.connections
    }

    pub fn connections_string(&self) -> String {
        format!("{:?}", self.connections)
    }
}

impl Index<Direction> for Position {
    type Output = Option<usize>;

    fn index(&self, direction: Direction) -> &Option<usize> {
        match direction {
            North => &self.north,
            East  => &self.east,
            South => &self.south,
            West  => &self.west,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.id == other.id
    }
}

impl Eq for Position {}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
