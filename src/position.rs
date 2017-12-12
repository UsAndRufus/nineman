#[derive(Debug)]
pub struct Position {
    // NB not using proper notation as it's a faff to work out with the way I'm generating the board
    pub id:    String,
    piece:     i8,
    pub north: Option<usize>,
    pub east:  Option<usize>,
    pub south: Option<usize>,
    pub west:  Option<usize>,
}

impl Position {
    pub fn new(id: String, north: Option<usize>, east: Option<usize>, south: Option<usize>,
               west:  Option<usize>) -> Position {
        return Position { id: id, piece: 0, north: north, south: south, east: east, west: west }
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

    pub fn piece(&self) -> i8 {
        self.piece
    }
}
