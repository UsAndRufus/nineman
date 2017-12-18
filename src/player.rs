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
        Player { name: name, id: id, colour: colour, bot: bot, pieces_left_to_place: Cell::new(2) }
    }

    pub fn make_move(&self) -> (String, String) {
        let mv = self.get_move();

        if self.is_placement() {
            self.pieces_left_to_place.set(self.pieces_left_to_place.get() - 1);
        }

        mv
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place.get() > 0
    }

    fn get_move(&self) -> (String, String) {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if self.is_placement() {
            ("".to_string(), input)
        } else {
            let mut split = input.split(",");
            (split.next().unwrap().to_string(), split.next().unwrap().to_string())
        }
    }

    pub fn get_pieces_left_to_place(&self) -> i8 {
        self.pieces_left_to_place.get()
    }
}
