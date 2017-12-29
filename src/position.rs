#[derive(Debug)]
pub struct Position {
    // NB not using proper notation as it's a faff to work out with the way I'm generating the board
    pub id:    String,
    piece:     i8,
    pub north: Option<usize>,
    pub east:  Option<usize>,
    pub south: Option<usize>,
    pub west:  Option<usize>,
    connections: Vec<usize>
}

impl Position {
    pub fn new(id: String, north: Option<usize>, east: Option<usize>, south: Option<usize>,
               west:  Option<usize>) -> Position {
        let connections = vec![north, east, south, west];
        println!("connections: {:?}", connections);
        return Position { id: id, piece: 0, north: north, south: south, east: east, west: west,
            connections: connections.iter().filter(|c| c.is_some()).map(|c| c.unwrap()).collect() }
    }

    pub fn blank(id: String) -> Position {
        return Position::new(id, None, None, None, None);
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
            println!("Checking for containing");
            println!("Contains? {}", self.connections.contains(other));
            println!("other: {}, connections: {:?}", other, self.connections);
            self.connections.contains(other)
        } else {
            println!("connected_to option empty");
            false
        }
    }
}
