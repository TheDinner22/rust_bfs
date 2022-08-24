//! # common breadth first search (bfs) logic
//! Find the most efficent path from point a to point b in some array or vec or struct representing
//! space
//!
//! ## how it works
//!
//! todo!()

use std::{error::Error, slice::Iter};

pub struct Path<'a, Cell, Move>
where
    Cell: PartialEq,
    Move: PartialEq + Copy,
{
    start_location: &'a Cell,
    path_taken: Vec<Move>,
}

impl<'a, Cell, Move> Path<'a, Cell, Move>
where
    Cell: PartialEq,
    Move: PartialEq + Copy,
{
    // create a new path
    pub fn new(start_location: &'a Cell, moves: Option<Vec<Move>>) -> Self {
        let path_taken;

        if let Some(mov) = moves {
            path_taken = mov;
        }
        else {
            path_taken = Vec::new();
        }

        Path {
            start_location,
            path_taken,
        }
    }

    // grab iterator of vector in the path struct
    fn iter(&self) -> Iter<Move> {
        self.path_taken.iter()
    }
}

pub trait PathAware<'a> { //todo get shortest working path
    type Cell: PartialEq;
    type Move: PartialEq + Copy;

    fn get_paths(&'a self) -> &'a Vec<Path<Self::Cell, Self::Move>>;
    fn create_path(&mut self, start_cell: &'a Self::Cell, moves: Option<Vec<Self::Move>>);
    fn set_paths(&mut self, new_paths: Vec<Path<'a, Self::Cell, Self::Move>>); // truncates old paths
    fn remove_path_by_index(&mut self, index_to_remove: usize);

    fn path_back_tracks(&self, _index_of_path_to_check: usize) -> bool {
        todo!(); // need to create a (get cells that compose path function)
    }

    fn get_path_from_index(&'a self, index: usize) -> &'a Path<Self::Cell, Self::Move> {
        let all_paths = self.get_paths();

        if index >= all_paths.len() { panic!("index that was passed to get_path_from_index was out of range") }

        &all_paths[index]
    }

    fn remove_paths_by_index(&mut self, path_indexes: Vec<usize>) {
        for (items_removed_so_far, index_to_remove) in path_indexes.iter().enumerate() {
            self.remove_path_by_index(index_to_remove - items_removed_so_far);
        }
    }

    fn check_and_trim_paths(&'a mut self) {
        let mut path_ids_to_remove = vec![];

        let all_paths = self.get_paths();
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
pub trait LocationAware<'a>: PathAware<'a> {
    fn list_all_moves(&self) -> Vec<Self::Move>;
    fn project_move(&self, start_cell: &Self::Cell, move_to_try: &Self::Move) -> Result<&Self::Cell, Box<dyn Error>>;

    fn get_cells_traversed_in_path(&'a self, index_of_path: usize) -> Vec<&Self::Cell>{
        let path = self.get_path_from_index(index_of_path);
        let mut cells_in_path = vec![path.start_location];

        for (i, move_made) in path.iter().enumerate() {
            cells_in_path.push(
                    self.project_move(cells_in_path[i], move_made)
                        .expect("The value returned from project to be the Ok varient becuase this is a move that WAS MADE")
                );
        }

        cells_in_path
    }

    fn get_a_paths_last_cell(&'a self, path_index: usize) -> &Self::Cell {
        self.get_cells_traversed_in_path(path_index)
            .pop()
            .expect("the path to have at least one cell in it (the starting cell)")
    }

    fn advance_and_split_all_paths(&mut self) {todo!()}
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
