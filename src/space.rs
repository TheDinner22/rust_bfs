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
    #[test]
    fn it_works(){
        assert_eq!(1+1, 2);
    }
}
