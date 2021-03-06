# nineman
Nine Men's Morris implementation in Rust.

## Rules/variants
* The "flying" variant is not (currently) implemented
* ~~Currently can mill pieces from other mills (to be fixed)~~ fixed

## Play
Clone repository and run `cargo run`. Currently play is against a random player.
Change the second player to be `Human` rather than `Random` to play against another human.
See my [greedyman repository](https://github.com/UsAndRufus/greedyman) for a better bot, and my [montyman repository](https://github.com/UsAndRufus/montyman) for a "better" bot that doesn't really work (you have to use `v0.2.1` for montyman too).

## Issues
* When there are no pieces not in mills, cannot mill
* Milling generally seems a bit dodgy, you occasionally can mill when you shouldn't be able to

### montyman (only works with v0.2.1)
* When using `montyman`, occasionally 3 mills will be created on a turn, even though I don't think that's possible (should panic but currently ignored)
* When using `montyman`, at the end of placement phase the bot will have a GameState where the placement_pieces are -1 (should panic but currently ignored)
* Separation of concerns isn't great between `nineman` and `montyman` (at least in v0.2.1)


## Pull requests
Very much welcome, but be aware this is my first Rust project so is definitely not idiomatic in places.
Also it was written in a hurry but has had a major refactor since so hopefully not too grim.

## crates.io
I plan to add this to [crates.io](https://crates.io/) at some point, but should probably add tests first.
