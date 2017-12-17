extern crate nineman;

use nineman::player::Player;
use nineman::game::Game;

fn main() {
    let p1 = Player::new(String::from("Dave"),   1, String::from("#000000"), false);

    let p2 = Player::new(String::from("Bertie"), 2, String::from("#110000"), true);

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    game.print();

    game.game_loop();
}
