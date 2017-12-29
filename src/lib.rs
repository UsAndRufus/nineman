extern crate itertools;

pub mod board;
pub mod player;
pub mod position;
pub mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
