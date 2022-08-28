pub trait HasId {
    type ID: PartialEq;

    fn get_uid(&self) -> Self::ID;
}
