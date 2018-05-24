use game::GameState;
use game::Ply;

// TODO: maybe make InputHandler return Plys instead?
pub trait InputHandler {
    fn give_new_game_state(&mut self, game_state: GameState);
    fn get_placement(&mut self, available_places: Vec<Ply>) -> Ply;
    fn get_move(&mut self, available_moves: Vec<Ply>) -> Ply;
    fn get_mill(&mut self, available_mills: Vec<Ply>) -> Ply;
    fn to_string(&self) -> String;
    fn set_player_id(&mut self, player_id: i8);
}
