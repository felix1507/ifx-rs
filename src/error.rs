pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    AxisToShort,
    UnconditionalState,
    CError,
}
