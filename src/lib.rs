//! # common breadth first search (bfs) logic
//! Find the most efficent path from point a to point b in some array or vec or struct representing
//! space
//!
//! ## how it works
//!
//! todo!()

use std::error::Error;

pub trait PathAware { //todo get shortest working path
    // cannot default impl this traits functions

    type Cell: PartialEq;
    type Move: PartialEq;

    // either a list of all possible moves (left, right, up, down, etc)...
    // or know as a path
    // todo: maybe the index should not be i32
    type CollectionOfMoves: PartialEq + IntoIterator<Item = Self::Move> + std::cmp::PartialOrd;

    fn get_paths(&self) -> Vec<&Self::CollectionOfMoves>;
    fn create_path(&mut self, path_to_create: Self::CollectionOfMoves);
    fn remove_paths_by_index(&mut self, path_indexes: Vec<usize>); // should use get_paths to get list of
                                                                   // possible paths to remove
    fn paths_intersect(&self, path_a: &Self::CollectionOfMoves, path_b: &Self::CollectionOfMoves) -> bool;
    fn path_back_tracks(&self, path_to_check: &Self::CollectionOfMoves) -> bool;

    fn check_and_trim_path(&mut self) {
        let all_paths = self.get_paths();

        let mut path_ids_to_remove = vec![];

        for a_index in 0..all_paths.len() {
            let path_a = &all_paths[a_index];

            // check if the path back tracks
            if self.path_back_tracks(path_a) {
                path_ids_to_remove.push(a_index);
                continue;
            }

            // check if it intersects with any another path
            for b_index in 0..all_paths.len() {
                let path_b = &all_paths[a_index];

                if path_b == path_a {continue;}

                if self.paths_intersect(path_a, path_b) {
                    if path_a > path_b {
                        path_ids_to_remove.push(a_index)
                    }
                    else{
                        path_ids_to_remove.push(b_index)
                    }
                }
            } 
        }

        self.remove_paths_by_index(path_ids_to_remove);
    }
}

// next you must fix this!!!
pub trait LocationAware: PathAware {
    const ALL_MOVES: Self::CollectionOfMoves;

    // fn project_move(&self, path: &Self::CollectionOfMoves, move_to_try: Self::Move) -> Result<&Self::Cell, Box<dyn Error>>;

    // fn create_paths_for_all_moves_from_cell(&mut self, path: &Self::CollectionOfMoves, all_possible_moves: Self::CollectionOfMoves){
    //
    // }
}


pub trait Bfs: LocationAware {
    fn bfs(&mut self, _start_cell: Self::Cell, _target_cell: Self::Cell) {//-> Vec<Self::Move>{
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
