extern crate nineman;

use nineman::game::Game;
use nineman::player::Player;
use nineman::player::Human;
use nineman::player::Random;

fn main() {
    let p1 = Player::new(String::from("Dave"), 1, Box::new(Human {player_id: 1}));

    let p2 = Player::new(String::from("Bertie"), 2, Box::new(Random {}));

    let mut game = Game::new(p1, p2);

    println!("{:?}", game);
    println!();

    game.game_loop();
}
