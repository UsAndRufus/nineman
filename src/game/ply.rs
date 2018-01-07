// A Ply is a "move", aka layer of game tree or each choice by a player
// See: https://en.wikipedia.org/wiki/Ply_(game_theory)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ply {
    Root,
    Placement{player_id: i8, piece_id: String},
    Mill{player_id: i8, piece_id: String},
    Move{player_id: i8, mv: (String, String)},
}
