extern crate nineman;

use nineman::board::Board;
use nineman::player::Player;

fn main() {
    let p1 = Player {
        name: String::from("Dave"),
        id: 1,
        colour: String::from("#000000"),
        bot: false,
        pieces_left_to_place: 9,
    };

    let p2 = Player {
        name: String::from("Bertie"),
        id: 2,
        colour: String::from("#110000"),
        bot: true,
        pieces_left_to_place: 9,
    };

    let board = Board::new(p1, p2);

    println!("{:?}", board);
    println!("{:?}", board.ids_to_positions);
    println!();

    board.print();

    println!("{:?}", board.current_player());
    board.make_move();
}
