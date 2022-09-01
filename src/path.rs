mod path_struct {
    #[derive(PartialEq, Debug)]
    pub struct Path<CellId, Move>
    where
        CellId: Copy,
        Move: Copy,
    {
        start_cell_id: CellId,
        moves_taken: Vec<Move>
    }

    impl<CellId, Move> Path<CellId, Move>
    where
        CellId: Copy,
        Move: Copy,
    {
        pub fn new(start_cell_id: CellId, moves: Option<Vec<Move>>) -> Self {
            Path { start_cell_id, moves_taken: moves.unwrap_or_default() }
        }

        pub fn clone_and_append(&self, move_to_append: Move) -> Self {
            let mut new_moves_taken = self.moves_taken.clone();
            new_moves_taken.push(move_to_append);
            
            Path { start_cell_id: self.start_cell_id, moves_taken: new_moves_taken }
        }
    }

    // fix these next!!!!
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //
    //     // vars needed by multiple tests
    //     #[derive(Clone, Copy, PartialEq, Debug)]
    //     enum ExampleMove {
    //         Up,
    //     }
    //
    //     #[derive(Debug, PartialEq)]
    //     struct ExampleCell {
    //         id: i32
    //     }
    //
    //     #[test]
    //     fn create_path() {
    //         let random_cell = ExampleCell { id: 22 };
    //
    //         let p: Path<ExampleCell, ExampleMove> = Path::new(
    //             &random_cell,
    //             Some(vec![])
    //         );
    //
    //         assert_eq!(p.moves_taken, vec![]);
    //         assert_eq!(p.start_cell, &ExampleCell{ id: 22});
    //     }
    //
    //     #[test]
    //     fn clone_path(){
    //         let random_cell = ExampleCell { id: 345 };
    //
    //         let p: Path<ExampleCell, ExampleMove> = Path::new(
    //             &random_cell,
    //             Some(vec![])
    //         );
    //
    //         let p_with_move_up: Path<ExampleCell, ExampleMove> = Path::new(
    //             &random_cell,
    //             Some(vec![ExampleMove::Up])
    //         );
    //
    //         let new_p = p.clone_and_append(ExampleMove::Up);
    //
    //         assert_eq!(new_p, p_with_move_up);
    //         assert_ne!(p, new_p);
    //     }
    // }
}

mod bfs_struct;
