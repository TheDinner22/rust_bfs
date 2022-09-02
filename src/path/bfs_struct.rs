use crate::space::RepresentsSpace;

use super::path_struct::Path;

use std::fmt::Debug;

#[derive(Debug)]
pub struct BfsAbleSpace<'space, CellId, Move, Space>
where
    CellId: Copy + PartialEq + Debug,
    Move: Copy + Debug,
    Space: RepresentsSpace<CellId = CellId, Move = Move>
{
    paths: Vec<Path<CellId, Move>>,
    space: &'space Space,
}

impl<'space, CellId, Move, Space> BfsAbleSpace<'space, CellId, Move, Space>
where
    CellId: Copy + PartialEq + Debug,
    Move: Copy + Debug,
    Space: RepresentsSpace<CellId = CellId, Move = Move> + Debug
{
    pub fn new (space: &'space Space) -> Self {
        BfsAbleSpace { paths: vec![], space }
    }

    fn create_path(&mut self, start_cell_id: CellId, moves: Option<Vec<Move>>) {
        self.paths.push( Path::new(start_cell_id, moves) )
    }

    fn get_paths(&self) -> &Vec<Path<CellId, Move>> {
        &self.paths
    }

    fn set_paths(&mut self, new_paths: Vec<Path<CellId, Move>>) {
        self.paths = new_paths
    }

    fn remove_path_by_index(&mut self, path_index: usize) {
        self.paths.remove(path_index);
    }

    fn get_path_from_index(&self, path_index: &usize) -> &Path<CellId, Move> {
        &self.get_paths()[*path_index]
    }

    fn remove_paths_by_indexes(&mut self, indexes_to_remove: Vec<usize>){
        indexes_to_remove
            .into_iter()
            .enumerate()
            .for_each(|(i, r)| { self.remove_path_by_index(r - i) });
    }

    fn path_backtracks(&self, path_index: &usize) -> bool {
        let cell_ids = self.get_cell_ids_in_path(path_index);

        (1..cell_ids.len()).any(|i| cell_ids[i..].contains(&cell_ids[i-1]))
    }

    // this might accidentally delete the shortest path, be careful
    fn check_and_trim_paths(&mut self) {
        self.remove_paths_by_indexes(
            // want this to be iter of only bad/dead paths
            self.get_paths()
                .iter()
                .enumerate()
                .filter(|(i, _)| self.path_backtracks(i)) // does it backtrack?
                .filter(|(i, _)| { // can it expand?
                    self.space.get_all_legal_moves_from_cell(
                        &self.get_paths_last_cell(i)
                    ).is_empty()
                })
                .map(|(i, _)| i)
                .collect()
        );
    }

    fn get_cell_ids_in_path(&self, path_index: &usize) -> Vec<CellId>{
        let path = self.get_path_from_index(path_index);

        let mut cell_ids = vec![path.start_cell_id];

        path.moves_taken
            .iter()
            .for_each(|m| {
                cell_ids.push(
                    self.space.project_move(
                        cell_ids.last().expect("cell_ids to never be empty"),
                        m
                    ).expect("move to be valid because it was made")
                )
            });

        cell_ids
    }

    fn get_paths_last_cell(&self, path_index: &usize) -> CellId{
        self
            .get_cell_ids_in_path(path_index)
            .pop()
            .expect("path to have at least one cell")
    }

    fn path_contains_target(&self, path_index: usize, target_cell_id: CellId) -> bool {
        self.get_cell_ids_in_path(&path_index).contains(&target_cell_id)
    }

    fn compute_new_paths(&self) -> Vec<Path<CellId, Move>>{
        self.paths
            .iter()
            .enumerate()
            .flat_map(|(i, p)| p.branch_into_multiple_paths(self.space.get_all_legal_moves_from_cell(&self.get_paths_last_cell(&i))))
            .collect()
    }

    pub fn bfs(&mut self, start_cell_id: CellId, target_cell_id: CellId) -> Option<Path<CellId, Move>> {
        self.set_paths(vec![]);

        // create the first path
        self.create_path(start_cell_id, Some(vec![]));

        loop {
            let paths = self.get_paths();

            // are we done?
            if paths.is_empty() { return None;}
            for (i, path) in self.get_paths().iter().enumerate() {
                if self.path_contains_target(i, target_cell_id) {
                    return Some(path.clone());
                }
            }

            // step paths
            self.set_paths(self.compute_new_paths());

            // trim paths
            self.check_and_trim_paths();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{uid::HasId, space::RepresentsSpace};

    use super::*;

    #[derive(Debug)]
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

        fn project_move(&self, _start_cell_id: &Self::CellId, _moov: &Self::Move) -> Option<Self::CellId> {
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
