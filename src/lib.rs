extern crate itertools;
extern crate term_painter;
extern crate rand;

pub mod board;
pub mod player;
pub mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
