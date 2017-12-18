use board::Board;
use player::Player;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub player1: Player,
    pub player2: Player,
    current_player_id: i8,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Game {
        Game {
            board: Board::new(),
            player1: player1,
            player2: player2,
            current_player_id: 1,
        }
    }

    pub fn print(&self) {
        self.board.print();
        println!("P1: {}; P2: {}",
            self.player1.get_pieces_left_to_place(), self.player2.get_pieces_left_to_place());
    }

    pub fn current_player(&self) -> &Player {
        match self.current_player_id {
            1 => &self.player1,
            2 => &self.player2,
            _ => panic!("Invalid player: {}", self.current_player_id),
        }
    }

    pub fn game_loop(&mut self) {
        loop {
            self.print();
            self.make_move();
        }
    }

    pub fn make_move(&mut self) {

        let (from, to) = self.get_move();
        let player_id = self.get_current_player_id();


        println!("from: {}, to: {}", from, to);

        if from.is_empty() {
            self.place_piece(player_id, to);
        }

        self.switch_player();
    }

    fn place_piece(&mut self, player_id: i8, piece_id: String) {
        let position = self.board.get_mut_position(piece_id);
        position.place(player_id);
    }

    fn get_move(&self) -> (String, String) {
        let player = self.current_player();
        player.make_move()
    }

    fn get_current_player_id(&self) -> i8 {
        self.current_player_id
    }

    fn switch_player(&mut self) {
        match self.current_player_id {
            1 => self.current_player_id = 2,
            2 => self.current_player_id = 1,
            _ => panic!("Invalid player id: {}", self.current_player_id),
        }
    }
}