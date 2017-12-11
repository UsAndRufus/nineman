use player::Player;

#[derive(Debug)]
pub struct Position<'a> {
    pub piece: Option<Player>,
    pub north: Option<&'a Position<'a>>,
    pub south: Option<&'a Position<'a>>,
    pub east:  Option<&'a Position<'a>>,
    pub west:  Option<&'a Position<'a>>,
}

impl<'a> Position<'a> {
    pub fn blank() -> Position<'a> {
        return Position{ piece: None, north: None, south: None, east: None, west: None };
    }
}

#[derive(Debug)]
pub struct Board<'a> {
    pub positions: Vec<Position<'a>>,
    pub players: Vec<Player>,
}

impl<'a> Board<'a> {
    pub fn new(player1: Player, player2: Player) -> Board<'a> {

        let mut board = Board {
            positions: Vec::new(),
            players: vec![player1, player2],
        };

        let (mut north, mut south, mut east, mut west)
            = (Position::blank(), Position::blank(), Position::blank(), Position::blank());
        for _layer in 0..3 {
            let nw = Position::blank();
            let ne = Position::blank();
            let sw = Position::blank();
            let se = Position::blank();

            let north_other = Position { piece: None, north: Some(&north), south: None, east: Some(&ne), west: Some(&sw) };

            board.positions.push(nw);
            board.positions.push(ne);
            board.positions.push(sw);
            board.positions.push(se);
        }

        return board;
    }
}
