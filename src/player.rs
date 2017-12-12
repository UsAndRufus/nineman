#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: i8,
    pub colour: String,
    pub bot: bool,
    pub pieces_left_to_place: i8,
}

impl Player {
    pub fn make_move(&self) -> (&str, &str) {
        //if self.is_placement() {
            return ("", "0nw");
        //}
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place > 0
    }
}
