use std::io;
use std::cell::Cell;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: i8,
    pub colour: String,
    pub bot: bool,
    pieces_left_to_place: Cell<i8>,
}

impl Player {
    pub fn new(name: String, id: i8, colour: String, bot: bool) -> Player {
        Player { name: name, id: id, colour: colour, bot: bot, pieces_left_to_place: Cell::new(9) }
    }

    pub fn make_move(&self) -> (String, String) {
        if self.is_placement() {
            self.pieces_left_to_place.set(self.pieces_left_to_place.get() - 1);
        }
        self.get_move()
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place.get() > 0
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

    pub fn get_pieces_left_to_place(&self) -> i8 {
        self.pieces_left_to_place.get()
    }
}
