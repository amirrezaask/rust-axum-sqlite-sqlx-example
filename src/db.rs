pub trait Database<ID, M> {
    type Error;

    fn store(item: M) -> Result<ID, Self::Error>;
    fn get(id: ID) -> Result<M, Self::Error>;
    fn update(id: ID, item: M) -> Result<(), Self::Error>;
    fn delete(id: ID) -> Result<(), Self::Error>;
}
