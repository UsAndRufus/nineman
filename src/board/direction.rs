use std::slice::Iter;

use self::Direction::*;

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 4] = [North, East, South, West];
        DIRECTIONS.into_iter()
    }
}
