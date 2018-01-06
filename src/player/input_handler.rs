use game::GameState;

pub trait InputHandler {
    fn update_game_state(&mut self, game: GameState);
    fn get_placement(&mut self, available_places: Vec<String>) -> String;
    fn get_move(&mut self, available_moves: Vec<(String, String)>) -> (String, String);
    fn get_mill(&mut self, available_mills: Vec<String>) -> String;
    fn to_string(&self) -> String;
}
