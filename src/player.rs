use std::io;
use std::cell::Cell;
use itertools::Itertools;

const WIN_SCORE: i8 = 1;
const STARTING_PIECES: i8 = 3;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: i8,
    pub bot: bool,
    score: Cell<i8>,
    pieces_left_to_place: Cell<i8>,
}

impl Player {
    pub fn new(name: String, id: i8, bot: bool) -> Player {
        Player { name: name, id: id, bot: bot, score: Cell::new(0),
                 pieces_left_to_place: Cell::new(STARTING_PIECES) }
    }

    pub fn make_move(&self) -> (String, String) {
        let mv = self.get_move();

        mv
    }

    pub fn mill(&self) -> String {
        println!("Mill! Select piece to destroy");
        self.get_input()
    }

    pub fn is_placement(&self) -> bool {
        self.pieces_left_to_place.get() > 0
    }

    pub fn place_piece(&self) {
        if self.is_placement() {
            self.pieces_left_to_place.set(self.pieces_left_to_place.get() - 1);
        }
    }

    pub fn increment_score(&self) {
        self.score.set(self.pieces_left_to_place.get() + 1);
    }

    pub fn has_won(&self) -> bool {
        self.score.get() >= WIN_SCORE
    }

    pub fn score(&self) -> i8 {
        self.score.get()
    }

    fn get_move(&self) -> (String, String) {
        if self.is_placement() {
            ("".to_string(), self.get_input())
        } else {
            loop {
                match self.get_input().split(",").map(|m| m.to_string()).next_tuple() {
                    Some(mv) => break mv,
                    None => println!("Invalid move, must be in format 0n,0e"),
                }
            }
        }
    }

    fn get_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input.trim().to_string().to_lowercase()
    }

    pub fn get_pieces_left_to_place(&self) -> i8 {
        self.pieces_left_to_place.get()
    }
}
