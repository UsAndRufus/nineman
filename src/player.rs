#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: i8,
    pub colour: String,
    pub bot: bool,
    pub pieces_left_to_place: i8,
}

impl Player {
    pub fn make_move(&self) -> (String, String) {
        //if self.is_placement() {
            return ("".to_string(), "0nw".to_string());
        //}
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place > 0
    }
}
