use game::Game;

pub trait InputHandler {
    fn update_game(&self, game: &Game);
    fn get_placement(&self, available_places: Vec<String>) -> String;
    fn get_move(&self, available_moves: Vec<(String, String)>) -> (String, String);
    fn get_mill(&self, available_mills: Vec<String>) -> String;
}
