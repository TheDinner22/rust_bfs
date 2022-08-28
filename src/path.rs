mod path_struct {
    use crate::uid;

    #[derive(PartialEq, Debug)]
    pub struct Path<'cell, Cell, Move>
    where
        Cell: uid::HasId,
        Move: Copy,
    {
        start_cell: &'cell Cell,
        moves_taken: Vec<Move>
    }

    impl<'cell, Cell, Move> Path<'cell, Cell, Move>
    where
        Cell: uid::HasId,
        Move: Copy,
    {
        pub fn new(start_cell: &'cell Cell, moves: Option<Vec<Move>>) -> Self {
            Path { start_cell, moves_taken: moves.unwrap_or_default() }
        }

        pub fn clone_and_append(&self, move_to_append: Move) -> Self {
            let mut new_moves_taken = self.moves_taken.clone();
            new_moves_taken.push(move_to_append);
            
            Path { start_cell: self.start_cell, moves_taken: new_moves_taken }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // vars needed by multiple tests
        #[derive(Clone, Copy, PartialEq, Debug)]
        enum ExampleMove {
            Up,
        }

        #[derive(Debug, PartialEq)]
        struct ExampleCell {
            id: i32
        }

        impl uid::HasId for ExampleCell {
            type ID = i32;

            fn get_uid(&self) -> Self::ID {
                self.id
            }
        }

        #[test]
        fn create_path() {
            let random_cell = ExampleCell { id: 22 };

            let p: Path<ExampleCell, ExampleMove> = Path::new(
                &random_cell,
                Some(vec![])
            );

            assert_eq!(p.moves_taken, vec![]);
            assert_eq!(p.start_cell, &ExampleCell{ id: 22});
        }

        #[test]
        fn clone_path(){
            let random_cell = ExampleCell { id: 345 };

            let p: Path<ExampleCell, ExampleMove> = Path::new(
                &random_cell,
                Some(vec![])
            );

            let p_with_move_up: Path<ExampleCell, ExampleMove> = Path::new(
                &random_cell,
                Some(vec![ExampleMove::Up])
            );

            let new_p = p.clone_and_append(ExampleMove::Up);

            assert_eq!(new_p, p_with_move_up);
            assert_ne!(p, new_p);
        }
    }
}

mod bfs_struct;
