extern crate nineman;

use nineman::board::Board;
use nineman::player::Player;

fn main() {
    let p1 = Player {
        name: String::from("Dave"),
        colour: String::from("#000000"),
        bot: false,
        pieces_left_to_place: 9,
    };

    let p2 = Player {
        name: String::from("Bertie"),
        colour: String::from("#110000"),
        bot: true,
        pieces_left_to_place: 9,
    };

    let board = Board {
        positions: Vec::new(),
        players: Vec::new(),
    };

    println!("p1 {:?}", p1);
    println!("p2 {:?}", p2);
    println!("{:?}", board);
}
