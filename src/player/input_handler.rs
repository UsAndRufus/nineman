pub trait InputHandler {
    fn get_placement(&self) -> String;
    fn get_move(&self) -> (String, String);
    fn get_mill(&self) -> String;
}
