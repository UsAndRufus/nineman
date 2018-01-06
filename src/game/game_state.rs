use std::fmt;

use board::Board;
use game::Game;
use player;

pub struct GameState {
    pub board: Board,
    pub current_player: i8,
    pub player1_score: i8,
    pub player2_score: i8,
    pub player1_pieces_to_place: i8,
    pub player2_pieces_to_place: i8,
}

impl GameState {
    pub fn from_game(game: &Game) -> Self {
        GameState {
            board: game.board.clone(),
            current_player: game.get_current_player_id(),
            player1_score: game.player1.score(),
            player2_score: game.player2.score(),
            player1_pieces_to_place: game.player1.get_pieces_left_to_place(),
            player2_pieces_to_place: game.player1.get_pieces_left_to_place(),
        }
    }

    pub fn winner(&self) -> Option<i8> {
        if self.player1_score >=  player::WIN_SCORE {
            return Some(1);
        }

        if self.player2_score >=  player::WIN_SCORE {
            return Some(2);
        }

        None
    }

    pub fn children(&self) -> Vec<Game> {
        Vec::new()
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f, "GameState(current_player: {}; p1: (s:{},p:{}), p2: (s:{},p:{}))",
                    self.current_player,
                    self.player1_score, self.player1_pieces_to_place,
                    self.player2_score, self.player2_pieces_to_place);

    }
}
