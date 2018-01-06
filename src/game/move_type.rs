#[derive(Clone, Debug)]
pub enum MoveType {
    Root,
    Placement{player_id: i8, piece_id: String},
    Mill{player_id: i8, piece_id: String},
    Move{player_id: i8, mv: (String, String)},
}
