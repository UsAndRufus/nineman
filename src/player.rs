use std::io;

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
        self.get_move()
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place > 0
    }

    fn get_move(&self) -> (String, String) {
        if self.is_placement() {
            let mut mv = String::new();

            io::stdin().read_line(&mut mv)
                .expect("Failed to read line");

            mv = mv.trim().to_string();
            ("".to_string(), mv)
        } else {
            ("".to_string(), "".to_string())
        }
    }
}
