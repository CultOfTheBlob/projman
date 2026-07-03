pub use crate::error::Error;

#[expect(unused)]
pub type Result<T> = core::result::Result<T, Error>;
