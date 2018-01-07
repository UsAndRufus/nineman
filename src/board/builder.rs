use std::collections::HashMap;
use std::collections::HashSet;

use board::Board;
use board::Position;
use board::Direction;
use board::Direction::*;

pub fn build() -> Board {
    let board = Board {
        positions: Vec::new(),
        ids_to_positions: HashMap::new(),
        p1_mills: HashSet::new(),
        p2_mills: HashSet::new(),
    };

    generate_positions(board)
}

pub fn generate_positions(mut board: Board) -> Board {

    let (mut prev_north, mut prev_south, mut prev_east, mut prev_west) = (None, None, None, None);
    for layer in 0..3 {

        let nw = board.new_blank_position(format!("{}nw", layer));
        let ne = board.new_blank_position(format!("{}ne", layer));
        let sw = board.new_blank_position(format!("{}sw", layer));
        let se = board.new_blank_position(format!("{}se", layer));

        let north = board.add_position(Position::new(format!("{}n", layer), prev_north, Some(ne), None, Some(nw)));
        let east  = board.add_position(Position::new(format!("{}e", layer), Some(ne), prev_east, Some(se), None));
        let south = board.add_position(Position::new(format!("{}s", layer), None, Some(se), prev_south, Some(sw)));
        let west  = board.add_position(Position::new(format!("{}w", layer), Some(nw), None, Some(sw), prev_west));

        board.positions[nw].add_neighbour(East,  Some(north));
        board.positions[nw].add_neighbour(South, Some(west));
        board.positions[ne].add_neighbour(West,  Some(north));
        board.positions[ne].add_neighbour(South, Some(east));
        board.positions[sw].add_neighbour(North, Some(west));
        board.positions[sw].add_neighbour(East,  Some(south));
        board.positions[se].add_neighbour(North, Some(east));
        board.positions[se].add_neighbour(West,  Some(south));

        add_connection_to_prev(&mut board, South, prev_north, north);
        add_connection_to_prev(&mut board, West,  prev_east, east);
        add_connection_to_prev(&mut board, North, prev_south, south);
        add_connection_to_prev(&mut board, East,  prev_west, west);


        prev_north = Some(north);
        prev_east  = Some(east);
        prev_south = Some(south);
        prev_west  = Some(west);
    }

    return board;
}

fn add_connection_to_prev(board: &mut Board, direction: Direction, from: Option<usize>, to: usize) {
    if let Some(p) = from {
        let f = board.positions.get_mut(p).unwrap();
        f.add_neighbour(direction, Some(to));
    }
}
