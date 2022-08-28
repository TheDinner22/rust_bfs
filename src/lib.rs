mod uid;

pub struct Path<'cell, Cell, Move>
where
    Cell: uid::IsUnique,
    Move: Copy,
{
    start_cell: &'cell Cell,
    moves_taken: Vec<Move>
}

impl<'cell, Cell, Move> Path<'cell, Cell, Move>
where
    Cell: uid::IsUnique,
    Move: Copy,
{
    fn new(start_cell: &'cell Cell, moves: Option<Vec<Move>>) -> Self {
        Path { start_cell, moves_taken: moves.unwrap_or(vec![]) }
    }

    fn clone_and_append(&self, move_to_append: Move) -> Self {
        let mut new_moves_taken = self.moves_taken.clone();
        new_moves_taken.push(move_to_append);
        
        Path { start_cell: self.start_cell, moves_taken: new_moves_taken }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1+1, 2);
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
