use player::Player;

#[derive(Debug)]
pub struct Position {
    pub piece: Player,
    pub north: Box<Position>,
    pub south: Box<Position>,
    pub east: Box<Position>,
    pub west: Box<Position>,
}

#[derive(Debug)]
pub struct Board {
    pub positions: Vec<Position>,
    pub players: Vec<Player>,
}
