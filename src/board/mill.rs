#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Mill {
    pub first: usize,
    pub second: usize,
    pub third: usize,
}

impl Mill {
    pub fn contains(&self, id: usize) -> bool {
        self.first == id || self.second == id || self.third == id
    }
}
