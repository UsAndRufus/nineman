use std::fmt;

use game::Ply;

pub const WIN_SCORE: i8 = 7;
pub const STARTING_SCORE: i8 = 0;
pub const STARTING_PIECES: i8 = 9;

#[derive(Clone, Eq, PartialEq)]
pub struct PlayerState {
    score: i8,
    pieces_left_to_place: i8,
}

impl PlayerState {
    pub fn at_beginning() -> Self {
        PlayerState {
            score: STARTING_SCORE,
            pieces_left_to_place: STARTING_PIECES,
        }
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place > 0
    }

    pub fn place_piece(&self) -> PlayerState{
        if self.is_placement() {
            PlayerState { score: self.score.clone(), pieces_left_to_place: self.pieces_left_to_place - 1 }
        } else {
            panic!("Tried to place piece when it was not placement!");
        }
    }

    pub fn increment_score(&self) -> PlayerState {
        PlayerState { score: self.score + 1, pieces_left_to_place: self.pieces_left_to_place.clone() }
    }

    pub fn has_won(&self, other_player_available_moves: Vec<Ply>,
                    other_player_placement: bool) -> bool {

        self.score >= WIN_SCORE ||
        (!other_player_placement && other_player_available_moves.is_empty())
    }

    pub fn score(&self) -> i8 {
        self.score
    }

    pub fn pieces_left_to_place(&self) -> i8 {
        self.pieces_left_to_place
    }
}

impl fmt::Debug for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f, "PlayerState (score: {}, pieces_left_to_place: {})",
                        self.score, self.pieces_left_to_place);

    }
}
