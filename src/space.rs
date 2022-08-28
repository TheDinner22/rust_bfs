use crate::uid::HasId;

pub trait RepresentsSpace {
    type CellId;
    type Cell: HasId<ID = Self::CellId>;

    fn get_cell_from_id(&self, cell_id: Self::CellId) -> &Self::Cell;

    fn get_id_from_cell (&self, cell: &Self::Cell) -> Self::CellId {
        cell.get_uid()
    }
}

#[cfg(test)]
mod tests {
    use crate::uid::HasId;

    use super::RepresentsSpace;

    struct Maze {
        cells: Vec<Cell>
    }

    impl Default for Maze {
        fn default() -> Self {
            Maze { cells: (0..20).into_iter().map(|n| Cell { id: n}).collect() }
        }
    }

    impl RepresentsSpace for Maze {
        type CellId = i32;

        type Cell = Cell;

        fn get_cell_from_id(&self, cell_id: Self::CellId) -> &Self::Cell {
            &self.cells[cell_id as usize]
        }
    }

    #[derive(Debug, PartialEq)]
    struct Cell {
        id: i32
    }

    impl HasId for Cell {
        type ID= i32;

        fn get_uid(&self) -> Self::ID {
            self.id
        }
    }

    #[test]
    fn create_maze_and_compare_ids(){
        let m = Maze::default();

        assert_ne!(m.get_id_from_cell(&m.cells[0]), m.get_id_from_cell(&m.cells[1]));
        assert_eq!(m.get_cell_from_id(0), &m.cells[0])
    }
}
