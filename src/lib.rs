//! # common breadth first search (bfs) logic
//! Find the most efficent path from point a to point b in some array or vec or struct representing
//! space
//!
//! ## how it works
//!
//! todo!()

use std::error::Error;

/// # trait for types that represent space
///
/// I recommend these types be some kind of struct (see types functions' docs to understand why)
///
pub trait LocationAware {
    /// # The smallest denominator of space
    ///
    /// The building block that makes up the space!
    ///
    /// ## examples
    ///
    /// blocks in minecraft, squares in chess, or i32 elements in a 1D array that represents a
    /// tic-tac-toe board
    ///
    /// ## requirements
    ///
    /// Must implement the PartialEq trait. This is so I can check to see if Cell A is equal to
    /// Cell B
    type Cell: PartialEq;

    /// # A way to encode different moves
    ///
    /// I recommend using an enum
    ///
    /// ## examples
    ///
    /// It depends on how many dimensions your space represents! Up, down, left, right, maybe diagonal, etc. 
    type Move;

    /// # get all of the cells at the forefront of a breadth first search
    ///
    /// This is why I recommend implementing this trait on a struct! You are responsible for
    /// somehow storing (and later updating) these values (probably as a property (of type Vec<Cell>) on the struct)
    ///
    /// # return type
    ///
    /// This function should return all of the cells at the forefront of a BFS. They can be
    /// returned in no particular order. By "at the forefront," I mean Cells that, in the
    /// next step of the BFS, are going to check for all possible moves around them (excluding the
    /// move that was previously made, which is back-tracking and will never result in the most
    /// efficent path).
    fn get_current_locations(&self) -> Vec<&Self::Cell>;

    /// # try to do every possible move from a provided location and return only the possible moves
    ///
    /// Whether or not a move is possible is up to you! Maybe you are coding a maze and the player
    /// should not be able to move through walls. This function is where you implement that logic!
    /// ***This function should not actually move anything***
    ///
    /// ## return type
    ///
    /// The function returns a list of possible moves in no particular order.
    fn get_all_moves_from_current_location(&self, current_location: &Self::Cell) -> Vec<Self::Move>;

    /// # attempt a move
    ///
    /// Try and move from one cell to another with one of the previously defined moves on the Move
    /// type. Again whether a move is "legal" or not is up to you!
    /// ***This is the only function that actually moves cells***
    ///
    /// ## return type
    ///
    /// This function returns a result of an OK Cell (where the current_cell ended up) or an Err std::error::Error trait object.
    /// 
    fn do_move(&mut self, current_location: &Self::Cell, mov: Self::Move) -> Result<&Self::Cell, Box<dyn Error>>;

    /// # 
    fn make_all_moves(all_possible_moves: Vec<Self::Move>);
    fn is_target_cell(cell_to_check: &Self::Cell) -> bool;
    fn generate_path_from_move_history(&self) -> Vec<Self::Move>;
}

pub trait Bfs: LocationAware {
    fn bfs(&self, target_cell: Self::Cell) -> Vec<Self::Move>{
        loop {
            // get current get_current_locations
            let current_cells = self.get_current_locations();

            // loop over all of them, making all possible moves at each step
            for current_cell in current_cells {
                // check to see if any of the current locations is the target location
                if current_cell == &target_cell {
                    self.generate_path_from_move_history()
                }
                else{continue;};

                // get all the moves allowed from this location
                let all_moves = self.get_all_moves_from_current_location(current_cell);

                // take all moves from current location
                Self::make_all_moves(all_moves);
            }
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
