// A Ply is a "move", aka layer of game tree or each choice by a player
// See: https://en.wikipedia.org/wiki/Ply_(game_theory)

use self::Ply::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ply {
    Root,
    Placement{player_id: i8, piece_id: String},
    Mill{player_id: i8, piece_id: String},
    Move{player_id: i8, mv: (String, String)},
}

impl Ply {
    pub fn player_id(&self) -> i8 {
        match *self {
            Root => 0,
            Placement{ref player_id, ..} => player_id.to_owned(),
            Mill{ref player_id, ..} => player_id.to_owned(),
            Move{ref player_id, ..} => player_id.to_owned(),
        }
    }
}
