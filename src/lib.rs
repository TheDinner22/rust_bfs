//! # common breadth first search (bfs) logic
//! Find the most efficent path from point a to point b in some array or vec or struct representing
//! space
//!
//! ## how it works
//!
//! todo!()

use std::error::Error;

pub trait PathAware {
    // cannot default impl this traits functions

    type Cell: PartialEq;
    type Move: PartialEq;

    // also know as a path
    type CollectionOfMoves: PartialEq + IntoIterator<Item = Self::Move> + std::cmp::PartialOrd;

    fn get_paths(&self) -> Vec<&Self::CollectionOfMoves>;
    fn create_path(&mut self, path_to_create: Self::CollectionOfMoves);
    fn remove_a_path(&mut self, path_to_remove: &Self::CollectionOfMoves);
    fn paths_intersect(&self, path_a: &Self::CollectionOfMoves, path_b: &Self::CollectionOfMoves) -> bool;
    fn path_back_tracks(&self, path_to_check: &Self::CollectionOfMoves) -> bool;

    fn check_and_trim_path(&mut self) {
        let all_paths = self.get_paths();

        let mut paths_to_remove = vec![];

        for path_a in all_paths {
            // check if the path back tracks
            if self.path_back_tracks(path_a) {
                paths_to_remove.push(path_a);
                continue;
            }

            // check if it intersects with any another path
            for path_b in all_paths {
                if path_b == path_a {continue;}

                if self.paths_intersect(path_a, path_b) {
                    if path_a > path_b {
                        paths_to_remove.push(path_a)
                    }
                    else{
                        paths_to_remove.push(path_b)
                    }
                }
            } 
        }

        for path in paths_to_remove {
            self.remove_a_path(path);
        }
    }
}

pub trait LocationAware {
    type Cell: PartialEq;

    type Move;

    type CollectionOfMoves: IntoIterator<Item = Self::Move>;


    fn project_move(&self, path: &Self::CollectionOfMoves, mov: Self::Move) -> Result<&Self::Cell, Box<dyn Error>>;

    fn make_all_moves_from_cell(&mut self, current_location: &Self::Cell, all_possible_moves: Self::CollectionOfMoves){
        for possible_move in all_possible_moves {
            let res_of_move = self.project_move(current_location, possible_move);

            if let Ok(new_location) = res_of_move {

            }
            else {
                // the move is illegal and can be ignored
                continue;
            }

        }
    }

    fn get_shortest_working_path(&self) -> Self::CollectionOfMoves;
}


pub trait Bfs: LocationAware {
    fn bfs(&mut self, start_cell: Self::Cell, target_cell: Self::Cell) {//-> Vec<Self::Move>{
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
