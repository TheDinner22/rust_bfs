// this is for testing only making sure remote works
/*
use rust_bfs::*;

struct TicTacToeBoard<'a> {
    paths: Vec<Path<'a, Square, Moves>>,
    squares: Vec<Square>,
}

impl TicTacToeBoard<'_> {
    fn set_square_state(&mut self, square_id: i32, new_state: SquareContent) {
        let square_to_change = &mut self.squares[square_id as usize];

        square_to_change.state = new_state;
    }
}

impl Default for TicTacToeBoard<'_> {
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

#[derive(PartialEq, Clone, Copy)]
enum Moves {
    Up,
    Down,
    Left,
    Right,
}

impl<'a> PathAware<'a> for TicTacToeBoard<'a> {
    type Cell = Square;

    type Move = Moves;

    fn get_paths(&self) -> &Vec<Path<'a, Self::Cell, Self::Move>> {
        &self.paths
    }

    fn create_path(&mut self, start_cell: &'a Self::Cell, moves: Option<Vec<Self::Move>>) {
        self.paths.push( Path::new(start_cell, moves) )
    }

    fn set_paths(&mut self, new_paths: Vec<Path<'a, Self::Cell, Self::Move>>) {
        self.paths = new_paths;
    }

    fn remove_path_by_index(&mut self, index_to_remove: usize) {
        self.paths.remove(index_to_remove);
    }
}

#[derive(Debug)]
struct MoveError {
    error_msg: String,
}

impl MoveError {
    fn new (error_msg: &str) -> Self {
        MoveError {
            error_msg: error_msg.to_string()
        }
    }

    fn boxed_new (error_msg: &str) -> Box<Self> {
        Box::new(MoveError::new(error_msg))
    }
}

impl std::fmt::Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

impl std::error::Error for MoveError {}

impl<'a> LocationAware<'a> for TicTacToeBoard<'a> {
    fn list_all_moves(&self) -> Vec<Self::Move> {
        vec![Moves::Up, Moves::Down, Moves::Left, Moves::Right]
    }

    fn project_move(&self, start_cell: &'a Self::Cell, move_to_try: &Self::Move) -> Result<&Self::Cell, Box<dyn std::error::Error>> {
        let res = &self.squares[0];
        Ok( res )
    }
}

fn main(){
    let mut board = TicTacToeBoard::default();

    board.set_square_state(0, SquareContent::X);
    board.set_square_state(1, SquareContent::O);
    board.set_square_state(8, SquareContent::X);

}
*/
fn main () { }
