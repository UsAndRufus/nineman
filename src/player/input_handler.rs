pub trait InputHandler {
    fn get_placement(&self, available_places: Vec<String>) -> String;
    fn get_move(&self) -> (String, String);
    fn get_mill(&self, available_mills: Vec<String>) -> String;
}
