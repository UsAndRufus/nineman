#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub id: i8,
    pub colour: String,
    pub bot: bool,
    pub pieces_left_to_place: i8,
}
