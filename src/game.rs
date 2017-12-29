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
            self.get_current_player().place_piece();
        }

        self.switch_player();
    }

    fn place_piece(&mut self, player_id: i8, piece_id: String) {
        let position = self.board.get_mut_position(piece_id);
        position.place(player_id);
    }

    fn get_move(&self) -> (String, String) {
        let player = self.current_player();

        let mut mv = ("".to_string(), "".to_string());
        let mut valid = false;

        while !valid {
            println!("{}", self.render_current_move());
            mv = player.make_move();
            valid = self.move_valid(&mv);
        }

        mv
    }
    fn render_current_move(&self) -> String {
        let player = self.current_player();

        let mv;
        if player.is_placement() {
            mv = "place";
        } else {
            mv = "move";
        }

        format!("P{} to {}:", player.id, mv)
    }

    fn move_valid(&self, mv: &(String, String)) -> bool {
        let (ref from, ref to) = *mv;
        (from == "" || self.board.is_valid_position(from)) &&
        self.board.is_valid_position(to) &&
        self.board.is_empty_position(to)
    }

    fn get_current_player_id(&self) -> i8 {
        self.current_player_id
    }

    fn get_current_player(&self) -> &Player {
        match self.current_player_id {
            1 => &self.player1,
            2 => &self.player2,
            _ => panic!("Invalid player id: {}", self.current_player_id),
        }
    }

    fn switch_player(&mut self) {
        match self.current_player_id {
            1 => self.current_player_id = 2,
            2 => self.current_player_id = 1,
            _ => panic!("Invalid player id: {}", self.current_player_id),
        }
    }
}
