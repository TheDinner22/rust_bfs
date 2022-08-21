// this is for testing only
use rust_bfs::*;

struct TicTacToeBoard {
    paths: Vec<Path<Square, Moves>>,
    squares: Vec<Square>,
}

impl TicTacToeBoard {
    fn set_square_state(&mut self, square_id: i32, new_state: SquareContent) {
        let square_to_change = &mut self.squares[square_id as usize];

        square_to_change.state = new_state;
    }
}

impl Default for TicTacToeBoard {
    fn default() -> Self {
        TicTacToeBoard {
            paths: vec![],
            squares: vec![
                Square::new(0),
                Square::new(1),
                Square::new(2),
                Square::new(3),
                Square::new(4),
                Square::new(5),
                Square::new(6),
                Square::new(7),
                Square::new(8),
            ],
        }
    }
}

#[derive(PartialEq)]
enum SquareContent {
    X,
    O,
    Empty,
}

#[derive(PartialEq)]
struct Square {
    state: SquareContent,
    id: i32,
}

impl Square {
    fn new(id: i32) -> Self {
        Square {
            state: SquareContent::Empty,
            id,
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum Moves {
    Up,
    Down,
    Left,
    Right,
}

impl PathAware for TicTacToeBoard {
    type Cell = Square;

    type Move = Moves;

    type CollectionOfMoves = Vec<Moves>;

    fn get_paths(&self) -> &Vec<Path<Self::Cell, Self::CollectionOfMoves>> {
        &self.paths
    }

    fn create_path_from_move(&mut self, start_cell: Self::Cell, first_move: Option<Self::Move>) {
        self.paths.push( Path::new(start_location, first_move) );
    }

    fn remove_path_by_index(&mut self, index_to_remove: usize) {
        self.paths.remove(index_to_remove);
    }
}

fn main(){
    let mut board = TicTacToeBoard::default();

    board.set_square_state(0, SquareContent::X);
    board.set_square_state(1, SquareContent::O);
    board.set_square_state(8, SquareContent::X);

}
