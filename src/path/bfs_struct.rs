use crate::space::RepresentsSpace;

use super::path_struct::Path;

pub struct BfsAbleSpace<'space, CellId, Move, Space>
where
    CellId: Copy,
    Move: Copy,
    Space: RepresentsSpace
{
    paths: Vec<Path<CellId, Move>>,
    space: &'space Space,
}

impl<'space, CellId, Move, Space> BfsAbleSpace<'space, CellId, Move, Space>
where
    CellId: Copy,
    Move: Copy,
    Space: RepresentsSpace,
{
    pub fn new (space: &'space Space) -> Self {
        BfsAbleSpace { paths: vec![], space }
    }

    pub fn create_path(&mut self, start_cell_id: CellId, moves: Option<Vec<Move>>) {
        self.paths.push( Path::new(start_cell_id, moves) )
    }

    pub fn get_paths(&self) -> &Vec<Path<CellId, Move>> {
        &self.paths
    }

    pub fn set_paths(&mut self, new_paths: Vec<Path<CellId, Move>>) {
        self.paths = new_paths
    }

    pub fn remove_path_by_index(&mut self, path_index: usize) {
        self.paths.remove(path_index);
    }

    pub fn get_path_from_index(&self, path_index: usize) -> &Path<CellId, Move> {
        &self.get_paths()[path_index]
    }

    pub fn remove_paths_by_indexes(&mut self, indexes_to_remove: Vec<usize>){
        indexes_to_remove
            .into_iter()
            .enumerate()
            .for_each(|(i, r)| { self.remove_path_by_index(r - i) });
    }
}

#[cfg(test)]
mod tests {
    use crate::{uid::HasId, space::RepresentsSpace};

    use super::*;

    struct ExampleSpace {
        cells: Vec<ExampleCell>
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    struct ExampleCell {
        id: i32
    }

    impl HasId for ExampleCell {
        type ID = i32;

        fn get_uid(&self) -> Self::ID {
            self.id
        }
    }

    impl Default for ExampleSpace {
        fn default() -> Self {
            ExampleSpace { cells: (0..20).into_iter().map(|n| ExampleCell { id: n }).collect() }
        }
    }

    impl RepresentsSpace for ExampleSpace {
        type CellId = i32;

        type Cell = ExampleCell;

        type Move = ExampleMove;

        fn list_all_moves(&self) -> Vec<Self::Move> {
            vec![ExampleMove::Up]
        }

        fn get_cell_from_id(&self, cell_id: Self::CellId) -> &Self::Cell {
            &self.cells[cell_id as usize]
        }

        fn project_move(&self, _start_cell_id: Self::CellId, _moov: &Self::Move) -> Self::CellId {
            todo!()
        }
    }

    #[derive(Clone, Copy)]
    #[derive(Debug)]
    enum ExampleMove {
        Up
    }

    #[test]
    fn create_path_in_bfs_space() {
        let a_space = ExampleSpace::default();
        let mut bfs: BfsAbleSpace<i32, ExampleMove, ExampleSpace> = BfsAbleSpace::new( &a_space );

        assert!(bfs.paths.is_empty());

        bfs.create_path(0, Some(vec![ExampleMove::Up]));

        assert_eq!(bfs.paths.len(), 1);

        bfs.remove_path_by_index(0);

        assert!(bfs.paths.is_empty());

        bfs.create_path(0, Some(vec![ExampleMove::Up]));

        assert_eq!(bfs.paths.len(), 1);

        bfs.set_paths(vec![]);

        assert!(bfs.paths.is_empty());
    }
}
