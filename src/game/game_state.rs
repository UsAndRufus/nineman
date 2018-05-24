use std::fmt;

use board;
use board::Board;

use player::PlayerState;

use game::Ply;
use game::Ply::*;
use game::switch_player_id;

#[derive(Clone, Eq, PartialEq)]
pub struct GameState {
    pub ply_to_get_here: Ply,
    pub next_ply: Ply, // NB ignore specific move here, just ply type and player_id. Is this bad? maybe. Or maybe blank piece_ids just mean unknown
    pub board: Board,
    pub current_player_id: i8,
    pub player1_state: PlayerState,
    pub player2_state: PlayerState,
}

impl GameState {

    pub fn at_beginning() -> Self {
        GameState {
            board: board::build(),
            ply_to_get_here: Root,
            next_ply: Root,
            current_player_id: 1,
            player1_state: PlayerState::at_beginning(),
            player2_state: PlayerState::at_beginning(),
        }
    }

    pub fn last_player_has_won(&self) -> bool {
        let new_player_state = self.current_player_state();
        let last_player_state = self.other_player_state();

        last_player_state.has_won(
                self.board.available_moves(self.current_player_id), // moves for new/current player
                new_player_state.is_placement())
    }

    pub fn children(&self) -> Vec<GameState> {
        assert!(self.next_ply.player_id() == self.current_player_id, "next_ply.player_id() should be same as current_player");

        // Could make all these calls to self.current_player_id just be in the methods?
        match self.next_ply {
            Placement{..} =>
                    self.board.available_places(self.current_player_id).into_iter()
                        .map(|p| self.place_piece(p)).collect(),
            Move{..} =>
                    self.board.available_moves(self.current_player_id).into_iter()
                        .map(|m| self.move_piece(m)).collect(),
            Mill{..} =>
                    self.board.available_mills(switch_player_id(self.current_player_id)).into_iter()
                        .map(|m| self.mill_piece(self.current_player_id, m)).collect(),
            _ => panic!("Found Ply::{:?}", self.next_ply),
        }
    }

    pub fn place_piece(&self, placement_ply: Ply) -> GameState {
        let mut game_state = self.clone();

        game_state.board.place_piece(placement_ply);
        game_state.ply_to_get_here = placement_ply;

        let player_id = placement_ply.player_id();

        let new_player_state = game_state.player_state(player_id).place_piece();
        game_state.update_player_state(player_id, new_player_state);

        give_new_game_state(&mut game_state, player_id);

        game_state
    }

    pub fn mill_piece(&self, player_id: i8, piece_id: String) -> GameState {
        let mut game_state = self.clone();

        game_state.board.perform_mill(piece_id.to_owned(), player_id);
        game_state.ply_to_get_here = Mill {player_id, piece_id};

        give_new_game_state(&mut game_state, player_id);

        game_state
    }

    pub fn move_piece(&self, move_ply: Ply) -> GameState {
        let mut game_state = self.clone();

        game_state.board.move_piece(move_ply);
        game_state.ply_to_get_here = move_ply;

        give_new_game_state(&mut game_state, move_ply.player_id());

        game_state
    }

    pub fn can_current_player_mill(&mut self) -> bool {
        let player_id = self.current_player_id;
        self.next_ply.is_mill() && (self.next_ply.player_id() == player_id)
    }

    pub fn can_mill_next(&mut self, player_id: i8) -> bool {
        self.board.update_mills(player_id)
    }

    fn new_next_ply(&mut self, player_id: i8, can_mill: bool) {
        let ply;
        // check if mill, player_id same
        if can_mill {
            ply = Mill {player_id: player_id, piece_id: "".to_string()};
        // if not mill, work out if placement, player_id is switched
        } else {
            let other_player_id = switch_player_id(player_id);

            if self.player_state(other_player_id).is_placement() {
                ply = Placement {player_id: other_player_id, piece_id: "".to_string()};
                // if not placement, must be move, player_id is switched
            } else {
                ply = Move {player_id: other_player_id, mv: ("".to_string(), "".to_string())};
            }
        }

        self.next_ply = ply;
    }

    fn player_state(&self, player_id: i8) -> &PlayerState {
        match player_id {
            1 => &self.player1_state,
            2 => &self.player2_state,
            _ => panic!("invalid player_id {}", player_id),
        }
    }

    pub fn current_player_state(&self) -> &PlayerState {
        self.player_state(self.current_player_id)
    }

    fn other_player_state(&self) -> &PlayerState {
        let other_id = switch_player_id(self.current_player_id);
        self.player_state(other_id)
    }

    fn update_player_state(&mut self, player_id: i8, player_state: PlayerState) {
        match player_id {
            1 => self.player1_state = player_state,
            2 => self.player2_state = player_state,
            _ => panic!("Invalid player_id {}", player_id),
        }
    }

    pub fn player_score(&self, player_id: i8) -> i8 {
        self.player_state(player_id).score()
    }

    pub fn print(&self) {
        self.board.print();
        println!("P1: p: {}, s: {}; P2: p: {}, s: {}",
            self.player1_state.pieces_left_to_place(),
            self.player1_state.score(),
            self.player2_state.pieces_left_to_place(),
            self.player2_state.score());
    }
}

// Could be better returning a GameState but doesn't make a huge difference
fn give_new_game_state(game_state: &mut GameState, player_id: i8) {
    let can_mill = game_state.can_mill_next(player_id);
    game_state.new_next_ply(player_id, can_mill);
    game_state.current_player_id = game_state.next_ply.player_id();

    // If this is a placement/move that leads to a mill, increment the score
    if can_mill {
        let new_player_state = game_state.player_state(player_id).increment_score();
        game_state.update_player_state(player_id, new_player_state);
    }
}

impl fmt::Debug for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        return write!(f, "GameState(ply_to_get_here: {:?}, next_ply: {:?}, current_player: {}; p1: (s:{},p:{}), p2: (s:{},p:{}))",
                    self.ply_to_get_here, self.next_ply,
                    self.current_player_id,
                    self.player1_state.score(), self.player1_state.pieces_left_to_place(),
                    self.player2_state.score(), self.player2_state.pieces_left_to_place());
    }
}
