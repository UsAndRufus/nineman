extern crate nineman;

use nineman::game::Game;
use nineman::player::Player;
use nineman::player::HumanInput;
use nineman::player::RandomInput;

fn main() {
    let p1 = Player::new(String::from("Dave"), 1, Box::new(HumanInput {}));

    let p2 = Player::new(String::from("Bertie"), 2, Box::new(RandomInput {}));

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    game.game_loop();
}
