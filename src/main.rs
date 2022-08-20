// this is for testing only
use rust_bfs::*;

struct TicTacToeBoard {
    paths: Vec<Vec<Moves>>,
    squares: Vec<Square>,
}

#[derive(PartialEq)]
enum Square {
    X,
    O,
    Empty,
}

#[derive(PartialEq, PartialOrd, Clone)]
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

    fn get_paths(&self) -> &Vec<Self::CollectionOfMoves> {
        &self.paths
    }

    fn create_path_from_move(&mut self, first_move: Self::Move) {
        self.paths.push(vec![first_move]);
    }

    fn remove_paths_by_index(&mut self, path_indexes: Vec<usize>) {
        for (items_removed_so_far, index_to_remove) in path_indexes.iter().enumerate() {
            self.paths.remove(index_to_remove - items_removed_so_far);
        }
    }

    fn paths_intersect(&self, path_a: &Self::CollectionOfMoves, path_b: &Self::CollectionOfMoves) -> bool {
        todo!()
    }

    fn path_back_tracks(&self, path_to_check: &Self::CollectionOfMoves) -> bool {
        todo!()
    }

}

fn main(){

}
