pub trait IsUnique {
    type ID: PartialEq;

    fn get_uid(&self) -> Self::ID;
}
