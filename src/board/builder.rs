use std::collections::HashMap;

use board::Board;
use board::position::Position;

pub fn build() -> Board {
    let board = Board {
        positions: Vec::new(),
        ids_to_positions: HashMap::new(),
    };

    generate_positions(board)
}

fn generate_positions(mut board: Board) -> Board {
    let (mut prev_north, mut prev_south, mut prev_east, mut prev_west) = (None, None, None, None);
    for layer in 0..3 {

        let nw = board.new_blank_position(format!("{}nw", layer));
        let ne = board.new_blank_position(format!("{}ne", layer));
        let sw = board.new_blank_position(format!("{}sw", layer));
        let se = board.new_blank_position(format!("{}se", layer));

        let north = board.add_position(Position::new(format!("{}n", layer), prev_north, None, Some(nw), Some(sw)));
        let east  = board.add_position(Position::new(format!("{}e", layer),Some(ne),Some(se), prev_east, None));
        let south = board.add_position(Position::new(format!("{}s", layer), None, prev_south, Some(sw), Some(se)));
        let west  = board.add_position(Position::new(format!("{}w", layer), Some(nw), Some(sw), None, prev_west));

        board.positions[nw].add_neighbour("east",  Some(north));
        board.positions[nw].add_neighbour("south", Some(west));
        board.positions[ne].add_neighbour("west",  Some(north));
        board.positions[ne].add_neighbour("south", Some(east));
        board.positions[sw].add_neighbour("north", Some(west));
        board.positions[sw].add_neighbour("east",  Some(south));
        board.positions[se].add_neighbour("north", Some(east));
        board.positions[se].add_neighbour("west",  Some(south));

        prev_north = Some(north);
        prev_east  = Some(east);
        prev_south = Some(south);
        prev_west  = Some(west);
    }

    return board;
}
