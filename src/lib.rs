//! # common breadth first search (bfs) logic
//! Find the most efficent path from point a to point b in some array or vec or struct representing
//! space
//!
//! ## how it works
//!
//! todo!()

use std::error::Error;
use std::cmp::PartialOrd;

pub struct Path<Cell, Move>
where
    Cell: PartialEq,
    Move: PartialEq,
{
    start_location: Cell,
    path_taken: Vec<Move>
}

impl<Cell, Move> Path<Cell, Move>
where
    Cell: PartialEq,
    Move: PartialEq,
{
    pub fn new(start_location: Cell, first_move: Option<Move>) -> Self {
        let mut path_taken = Vec::new();

        if let Some(mov) = first_move {
            path_taken.push(mov);
        }

        Path {
            start_location,
            path_taken,
        }
    }
}

pub trait PathAware { //todo get shortest working path
    // cannot default impl this traits functions

    type Cell: PartialEq;
    type Move: PartialEq;

    // either a list of all possible moves (left, right, up, down, etc)...
    // or know as a path
    // todo: maybe the index should not be i32
    type CollectionOfMoves: PartialEq + PartialOrd;

    fn get_paths(&self) -> &Vec<Path<Self::Cell, Self::CollectionOfMoves>>;
    fn create_path(&mut self, start_cell: Self::Cell, first_move: Option<Self::Move>);
    fn remove_path_by_index(&mut self, index_to_remove: usize);

    fn path_back_tracks(&self, _index_of_path_to_check: usize) -> bool {
        todo!();
    }

    fn get_path_from_index(&self, index: usize) -> &Path<Self::Cell, Self::CollectionOfMoves> {
        let all_paths = self.get_paths();

        if index >= all_paths.len() { panic!("index that was passed to get_path_from_index was out of range") }

        &all_paths[index]
    }

    fn remove_paths_by_index(&mut self, path_indexes: Vec<usize>) {
        for (items_removed_so_far, index_to_remove) in path_indexes.iter().enumerate() {
            self.remove_path_by_index(index_to_remove - items_removed_so_far);
        }
    }

    fn check_and_trim_paths(&mut self) {
        let all_paths = self.get_paths();

        let mut path_ids_to_remove = vec![];

        for path_index in 0..all_paths.len() {
            // check if the path back tracks
            if self.path_back_tracks(path_index) {
                path_ids_to_remove.push(path_index);
            }
        }

        self.remove_paths_by_index(path_ids_to_remove);
    }
}

// next you must fix this!!!
pub trait LocationAware: PathAware {
    const ALL_MOVES: Self::CollectionOfMoves;

    fn project_move(&self, start_cell: Self::Cell, move_to_try: &Self::Move) -> Result<&Self::Cell, Box<dyn Error>>;

    fn get_paths_last_cell(&self, path_index: usize){ //-> Self::Cell {
        let path = self.get_path_from_index(path_index);
        let paths_last_cell = ;
    }

    fn advance_all_paths(&mut self) {
        // get all paths and loop over them
        let all_paths = self.get_paths();

        for path in all_paths {


        }
    }
}


pub trait Bfs: LocationAware {
    fn bfs(&mut self, _start_cell: Self::Cell, _target_cell: Self::Cell) {
        todo!();
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn dummy_implementation_of_array_as_grid() {
        let mut grid = vec![];
        for i in 0..100 {
            grid.push(i)
        }
    }
}

// some ideas and sudo code
//
// need an array or array like struct or something (must somehow, loose-ly represent some space, 2d, 3d, etc.)
// (must have some smallest denominator of space i.e
// a block in minecraft, a square in chess, or a cell in conways game of life)
// that implements a BFS trait and a location trait
// location trait: somehow keep track of position in the space being represented and be able to
// move through that space (moving is a seperate trait??) 
// BFS trait: where you can call the trait to get all the possible moves from your current "location," 
// then spawn several locations (how can i spawn a trait) that do all possible moves and repeat
// until reaching some specified location the path to that location is the path of "least
// resistance" and is returned or something
//
// i have realized that there are some things about bfs i can assert! For example there must be
// paths that are vec<Move>. A path can only ever contain a cell once (they are sets no vecs). If
// two paths cross, the larger one is invalid
// i need to implement behavior for paths 
//
//
