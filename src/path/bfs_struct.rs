use crate::uid;

use super::path_struct::Path;

pub struct BfsAbleSpace<'cell, Cell, Move, Space>
where
    Cell: uid::HasId,
    Move: Copy
{
    paths: Vec<Path<'cell, Cell, Move>>,
    space: Space,
}

impl<'cell, Cell, Move, Space> BfsAbleSpace<'cell, Cell, Move, Space>
where
    Cell: uid::HasId,
    Move: Copy
{
    pub fn new (space: Space) -> Self {
        BfsAbleSpace { paths: vec![], space }
    }

    pub fn create_path(&mut self, start_cell: &'cell Cell, moves: Option<Vec<Move>>) {
        self.paths.push( Path::new(start_cell, moves) )
    }

    pub fn get_paths(&self) -> &Vec<Path<'cell, Cell, Move>> {
        &self.paths
    }

    pub fn set_paths(&mut self, new_paths: Vec<Path<'cell, Cell, Move>>) {
        self.paths = new_paths
    }

    pub fn remove_path_by_index(&mut self, path_index: usize) {
        self.paths.remove(path_index);
    }

    pub fn get_path_from_index(&self, path_index: usize) -> &Path<'cell, Cell, Move> {
        &self.get_paths()[path_index]
    }

    pub fn remove_paths_by_indexes(&mut self, indexes_to_remove: Vec<usize>){
        indexes_to_remove
            .into_iter()
            .enumerate()
            .for_each(|(i, r)| { self.remove_path_by_index(r - i) });
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::uid::IsUnique;
//
//     // use super::*;
//
//     struct ExampleSpace {
//         cells: Vec<ExampleCell>
//     }
//
//     #[derive(PartialEq, Debug)]
//     struct ExampleCell {
//         id: i32
//     }
//
//     impl IsUnique for ExampleCell {
//         type ID = i32;
//
//         fn get_uid(&self) -> Self::ID {
//             self.id
//         }
//     }
//
//     impl Default for ExampleSpace {
//         fn default() -> Self {
//             ExampleSpace { cells: (0..20).into_iter().map(|n| ExampleCell { id: n }).collect() }
//         }
//     }
//
//     #[derive(Clone, Copy)]
//     #[derive(Debug)]
//     enum ExampleMove {
//         Up
//     }
//
//     #[test]
//     fn create_path_in_bfs_space() {
//         todo!()
//         make this when the space trait in impl
//         let mut bfs: BfsAbleSpace<ExampleCell, ExampleMove, ExampleSpace> = BfsAbleSpace::new( ExampleSpace::default() );
//
//         assert!(bfs.paths.is_empty());
//
//         bfs.create_path(&bfs.space.cells[0], Some(vec![ExampleMove::Up]))
//     }
// }

