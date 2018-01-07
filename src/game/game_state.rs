use std::fmt;

use board::Board;
use game::Game;
use player;

use game::Ply;
use game::Ply::*;
use game::switch_player_id;

#[derive(Clone, Eq, PartialEq)]
pub struct GameState {
    pub ply_to_get_here: Ply,
    pub next_ply: Ply, // NB ignore specific move here, just ply type and player_id. Is this bad? maybe. Or maybe blank piece_ids just mean unknown
    pub board: Board,
    pub current_player: i8,
    pub player1_score: i8,
    pub player2_score: i8,
    pub player1_pieces_to_place: i8,
    pub player2_pieces_to_place: i8,
}

impl GameState {
    pub fn from_game(game: &Game, next_ply: Ply) -> Self {
        GameState {
            ply_to_get_here: Root,
            next_ply: next_ply,
            board: game.board.clone(),
            current_player: game.get_current_player_id(),
            player1_score: game.player1.score(),
            player2_score: game.player2.score(),
            player1_pieces_to_place: game.player1.get_pieces_left_to_place(),
            player2_pieces_to_place: game.player1.get_pieces_left_to_place(),
        }
    }

    pub fn winner(&self) -> Option<i8> {
        if self.player1_score >= player::WIN_SCORE {
            return Some(1);
        }

        if self.player2_score >= player::WIN_SCORE {
            return Some(2);
        }

        None
    }

    // NB: only works for placement currently
    pub fn children(&self) -> Vec<GameState> {
        assert!(self.next_ply.player_id() == self.current_player, "next_ply.player_id() should be same as current_player");
        let current_player_id = self.current_player;

        match self.next_ply {
            Placement{..} =>
                    self.board.available_places().into_iter()
                        .map(|p| self.placement_child(current_player_id, p)).collect(),
            Move{..} =>
                    self.board.available_moves(current_player_id).into_iter()
                        .map(|m| self.move_child(current_player_id, m)).collect(),
            Mill{..} =>
                    self.board.available_mills(current_player_id).into_iter()
                        .map(|m| self.mill_child(current_player_id, m)).collect(),
            _ => panic!("Found Ply::{:?}", self.next_ply),
        }
    }

    pub fn placement_child(&self, player_id: i8, piece_id: String) -> GameState {
        let mut game_state = self.clone();

        game_state.board.place_piece(player_id, piece_id.to_owned());
        game_state.ply_to_get_here = Placement {player_id, piece_id};

        update_game_state(&mut game_state, player_id);

        game_state
    }

    pub fn mill_child(&self, player_id: i8, piece_id: String) -> GameState {
        let mut game_state = self.clone();

        game_state.board.perform_mill(piece_id.to_owned(), player_id);
        game_state.ply_to_get_here = Mill {player_id, piece_id};

        update_game_state(&mut game_state, player_id);

        game_state
    }

    pub fn move_child(&self, player_id: i8, mv: (String, String)) -> GameState {
        let mut game_state = self.clone();

        game_state.board.move_piece(player_id, mv.0.to_owned(), mv.1.to_owned());
        game_state.ply_to_get_here = Move {player_id, mv};

        update_game_state(&mut game_state, player_id);

        game_state
    }

    fn new_next_ply(&mut self) {
        let ply;
        // check if mill, player_id same
        if self.board.can_mill() {
            ply = Mill {player_id: self.current_player, piece_id: "".to_string()};
        // if not mill, work out if placement, player_id is switched
        } else {
            let player_id = switch_player_id(self.current_player);

            if self.is_placement(player_id) {
                ply = Placement {player_id: player_id, piece_id: "".to_string()};
                // if not placement, must be move, player_id is switched
            } else {
                ply = Move {player_id: player_id, mv: ("".to_string(), "".to_string())};
            }
        }

        self.next_ply = ply;
    }

    fn is_placement(&self, player_id: i8) -> bool {
        let pieces_left;
        match player_id {
            1 => pieces_left = self.player1_pieces_to_place,
            2 => pieces_left = self.player2_pieces_to_place,
            _ => panic!("invalid player_id {}", player_id),
        }

        pieces_left > 0
    }
}

fn update_game_state(game_state: &mut GameState, player_id: i8) {
    game_state.board.update_mills(player_id);
    game_state.new_next_ply();
    game_state.current_player = game_state.next_ply.player_id();
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f, "GameState(current_player: {}; p1: (s:{},p:{}), p2: (s:{},p:{}))",
                    self.current_player,
                    self.player1_score, self.player1_pieces_to_place,
                    self.player2_score, self.player2_pieces_to_place);
    }
}
