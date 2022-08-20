//! # common breadth first search (bfs) logic
//! Find the most efficent path from point a to point b in some array or vec or struct representing
//! space
//!
//! ## how it works
//!
//! todo!()

pub trait LocationAware {
    type Cell: PartialEq; // the smallest denominator of space
    type Move; // a way to encode different moves (up down left right, maybe diagonal, etc)

    fn get_current_location() -> Self::Cell;
    fn get_all_moves_from_current_location(current_location: Self::Cell) -> Vec<Self::Move>;
    fn make_all_moves(all_possible_moves: Vec<Self::Move>);
    fn is_target_cell(cell_to_check: &Self::Cell) -> bool;
}

pub trait Bfs: LocationAware {
    fn bfs(&self, mut start_cell: Self::Cell, target_cell: Self::Cell) { //Vec<Self::Move>{
        loop {
            // get all the moves allowed from this location
            let all_moves = Self::get_all_moves_from_current_location(start_cell);

            // take all moves from current location
            Self::make_all_moves(all_moves);

            // check to see if the new current location is the target location
            let new_current_cell = Self::get_current_location();
            if new_current_cell == target_cell {
                break;
            }
            start_cell = new_current_cell;
        }
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
//
//
