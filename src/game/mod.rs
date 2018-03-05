use term_painter::Color::*;
use term_painter::ToStyle;

mod game_state;
mod ply;

pub use self::game_state::GameState;
pub use self::ply::Ply;

use player::Player;
use board::Board;

#[derive(Debug)]
pub struct Game {
    pub current_state: GameState,
    pub player1: Player,
    pub player2: Player,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        let mut game = Game {
            current_state: GameState::at_beginning(),
            player1: player1,
            player2: player2,
        };
        game.update_input_handlers();
        game.player1.set_input_handler_player_id(1);
        game.player2.set_input_handler_player_id(2);
        game
    }

    pub fn print(&self) {
        self.current_state.print();
    }

    pub fn game_loop(&mut self) -> i8 {
        loop {
            self.print();
            self.make_move();
            self.mill();

            if self.current_state.current_player_has_won() {
                break
            }

            self.update_input_handlers();
        }

        self.end_game()
    }

    fn update_input_handlers(&mut self) {
        self.update_input_handler_for(1);
        self.update_input_handler_for(2);
    }

    // TODO: this should just pass current state
    fn update_input_handler_for(&mut self, player_id: i8) {
        let next_ply = self.next_ply();
        let game_state = GameState::from_game(&self, next_ply);

        let player = self.get_player_mut(player_id);
        player.give_new_game_state(game_state);

    }

    // Whenever we update, it will be after a mill so we don't need to bother with Mill plies
    // piece_ids are just "" because they are unknown so far
    fn next_ply(&self) -> Ply {
        let player_id = self.get_current_player_id();
        if self.get_current_player().is_placement() {
            Ply::Placement {player_id: player_id, piece_id: "".to_string()}
        } else {
            Ply::Move {player_id: player_id, mv: ("".to_string(),"".to_string())}
        }
    }

    fn mill(&mut self) {
        let can_mill = self.board.update_mills(self.get_current_player_id());

        if can_mill {
            self.board.print();
            let available_mills = self.board.available_mills(self.get_other_player().id);
            let position = self.get_current_player_mut().mill(available_mills);
            self.board.perform_mill(position, self.get_current_player_id());
            self.get_current_player().increment_score();
        }
    }

    fn make_move(&mut self) {
        let player_id = self.get_current_player_id();

        if self.get_current_player().is_placement() {
            let placement = self.get_placement();
            self.board.place_piece(player_id, placement);
            self.get_current_player().place_piece();
        } else {
            let (from, to) = self.get_move();
            self.board.move_piece(player_id, from, to);
        }
    }

    // TODO: print the proper score
    fn end_game(&self) -> i8 {
        let winner = self.get_current_player();
        let loser = self.get_other_player();
        let winner_name;
        match winner.id {
            1 => winner_name = Green.paint(winner.name.to_owned()),
            2 => winner_name = Red.paint(winner.name.to_owned()),
            _ => panic!("Unknown player id: {}", winner.id),
        }
        // println!("Congratulations, {} (Player {})! You win with a score of {}", winner_name, winner.id, winner.score());
        // println!("Commiserations, {} (Player {}). You lose with a score of {}", loser.name, loser.id, loser.score());
        println!("Congratulations, {} (Player {})! You win with a score of x", winner_name, winner.id);
        println!("Commiserations, {} (Player {}). You lose with a score of y", loser.name, loser.id);

        winner.id
    }

    fn get_move(&mut self) -> (String, String) {
        let available_moves = self.board().available_moves(self.get_current_player_id());

        let player = self.get_current_player_mut();

        player.get_move(available_moves)
    }

    fn get_placement(&mut self) -> String {
        let available_places = self.board().available_places();

        let player = self.get_current_player_mut();

        player.get_placement(available_places)
    }

    fn render_current_move(&self) -> String {
        let mv;
        if self.current_state.current_player_state().is_placement() {
            mv = "place";
        } else {
            mv = "move";
        }

        format!("P{} to {}:", self.get_current_player_id(), mv)
    }

    pub fn get_current_player_id(&self) -> i8 {
        self.current_state.current_player_id
    }

    pub fn get_player_mut(&mut self, player_id: i8) -> &mut Player {
        match player_id {
            1 => &mut self.player1,
            2 => &mut self.player2,
            _ => panic!("Invalid player id: {}", self.get_current_player_id()),
        }
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        let player_id = self.get_current_player_id();
        self.get_player_mut(player_id)
    }

    fn get_current_player(&self) -> &Player {
        match self.get_current_player_id() {
            1 => &self.player1,
            2 => &self.player2,
            _ => panic!("Invalid player id: {}", self.get_current_player_id()),
        }
    }

    fn get_other_player(&self) -> &Player {
        match self.get_current_player_id() {
            2 => &self.player1,
            1 => &self.player2,
            _ => panic!("Invalid player id: {}", self.get_current_player_id()),
        }
    }

    fn board(&mut self) -> Board {
        self.current_state.board
    }
}

pub fn switch_player_id(player_id: i8) -> i8 {
    match player_id {
        1 => 2,
        2 => 1,
        _ => panic!("invalid player_id {}", player_id),
    }
}
