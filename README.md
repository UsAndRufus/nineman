# nineman
Nine Men's Morris implementation in Rust.

## Rules/variants
* The "flying" variant is not (currently) implemented
* ~~Currently can mill pieces from other mills (to be fixed)~~ fixed

## Play
Clone repository and run `cargo run`. Currently play is against a random player.
Change the second player to be `Human` rather than `Random` to play against another human.
See my [montyman repository](https://github.com/UsAndRufus/montyman) for a better bot.

## Issues
* When using `montyman`, occasionally 3 mills will be created on a turn, even though I don't think that's possible (should panic but currently ignored)
* When using `montyman`, at the end of placement phase the bot will have a GameState where the placement_pieces are -1 (should panic but currently ignored)
* Separation of concerns isn't great between `nineman` and `montyman` (as you can probably see from the above issues)
* Available moves methods should probably return Vecs of `Ply`s
* The code is a bit grim

## Pull requests
Very much welcome, but be aware this is my first Rust project and is all done fairly hastily so I can write the bot.

## crates.io
I plan to add this to [crates.io](https://crates.io/) at some point, but should probably add tests. Also I am too busy with the bot to work out the procedure to publish to the repository!
