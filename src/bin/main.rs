extern crate nineman;

use nineman::player::Player;
use nineman::game::Game;

fn main() {
    let p1 = Player::new(String::from("Dave"), 1);

    let p2 = Player::new(String::from("Bertie"), 2);

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    game.game_loop();
}
