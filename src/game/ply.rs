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

    pub fn piece_id(&self) -> String {
        match *self {
            Placement{ref piece_id, ..} => piece_id.to_owned(),
            Mill{ref piece_id, ..} => piece_id.to_owned(),
            _ => panic!("Ply does not contain field piece_id!"),
        }
    }

    pub fn mv(&self) -> (String, String) {
        match *self {
            Move{ref mv, ..} => mv.to_owned(),
            _ => panic!("Ply is not a move!"),
        }
    }

    pub fn is_mill(&self) -> bool {
        match *self {
            Mill{..} => true,
            _ => false,
        }
    }
}
