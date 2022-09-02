mod path_struct {
    #[derive(PartialEq, Debug)]
    pub struct Path<CellId, Move>
    where
        CellId: Copy + PartialEq,
        Move: Copy,
    {
        pub start_cell_id: CellId,
        pub moves_taken: Vec<Move>
    }

    impl<CellId, Move> Clone for Path<CellId, Move>
    where
        CellId: Copy + PartialEq,
        Move: Copy,
    {
        fn clone(&self) -> Self {
            Path { start_cell_id: self.start_cell_id, moves_taken: self.moves_taken.clone() }
        }
    }

    impl<CellId, Move> Path<CellId, Move>
    where
        CellId: Copy + PartialEq,
        Move: Copy,
    {
        pub fn new(start_cell_id: CellId, moves: Option<Vec<Move>>) -> Self {
            Path { start_cell_id, moves_taken: moves.unwrap_or_default() }
        }

        fn append_to_path(&mut self, move_to_append: Move){
            self.moves_taken.push(move_to_append);
        }

        fn clone_and_append(&self, move_to_append: Move) -> Self {
            let mut new_path = self.clone();
            new_path.append_to_path(move_to_append);

            new_path
        }

        pub fn branch_into_multiple_paths(&self, moves_to_append: Vec<Move>) -> Vec<Self> {
            moves_to_append
                .into_iter()
                .map(|m| self.clone_and_append(m))
                .collect()
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

        #[test]
        fn create_path() {
            let _random_cell = ExampleCell { id: 22 };

            let p: Path<i32, ExampleMove> = Path::new(
                22,
                Some(vec![])
            );

            assert_eq!(p.moves_taken, vec![]);
            assert_eq!(p.start_cell_id, 22);
        }

        #[test]
        fn clone_path(){
            let _random_cell = ExampleCell { id: 345 };

            let p: Path<i32, ExampleMove> = Path::new(
                345,
                Some(vec![])
            );

            let p_with_move_up: Path<i32, ExampleMove> = Path::new(
                345,
                Some(vec![ExampleMove::Up])
            );

            let new_p = p.clone_and_append(ExampleMove::Up);

            assert_eq!(new_p, p_with_move_up);
            assert_ne!(p, new_p);
        }
    }
}

pub mod bfs_struct;

