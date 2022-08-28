use crate::uid;

use super::path_struct::Path;

pub struct BfsAbleSpace<'cell, Cell, Move, Space>
where
    Cell: uid::IsUnique,
    Move: Copy
{
    paths: Vec<Path<'cell, Cell, Move>>,
    space: Space,
}

impl<'cell, Cell, Move, Space> BfsAbleSpace<'cell, Cell, Move, Space>
where
    Cell: uid::IsUnique,
    Move: Copy
{
    fn create_path(&mut self, start_cell: &'cell Cell, moves: Option<Vec<Move>>) {
        self.paths.push( Path::new(start_cell, moves) )
    }

    fn get_paths(&self) -> &Vec<Path<'cell, Cell, Move>> {
        &self.paths
    }

    fn set_paths(&mut self, new_paths: Vec<Path<'cell, Cell, Move>>) {
        self.paths = new_paths
    }

    fn remove_path_by_index(&mut self, path_index: usize) {
        self.paths.remove(path_index);
    }

    fn get_path_from_index(&self, path_index: usize) -> &Path<'cell, Cell, Move> {
        &self.get_paths()[path_index]
    }

    fn remove_paths_by_indexes(&mut self, indexes_to_remove: Vec<usize>){
        indexes_to_remove
            .into_iter()
            .enumerate()
            .for_each(|(i, r)| { self.remove_path_by_index(r - i) });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){
        assert_eq!(1+1, 2);
    }
}

