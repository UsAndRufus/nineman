extern crate nineman;

use nineman::board::Board;
use nineman::player::Player;

fn main() {
    let p1 = Player::new(String::from("Dave"),   1, String::from("#000000"), false);

    let p2 = Player::new(String::from("Bertie"), 2, String::from("#110000"), true);

    let mut board = Board::new(p1, p2);

    println!("{:?}", board);
    println!("{:?}", board.ids_to_positions);
    println!();

    board.print();

    board.game_loop();
}
