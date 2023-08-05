mod exception;
pub use exception::YoutubeError;

pub type AppResult<T> = Result<T, exception::YoutubeError>;

pub mod extract;

pub mod helpers;

pub mod parser;

#[allow(unused)]
mod test_utils;
