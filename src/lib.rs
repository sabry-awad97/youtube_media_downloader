mod exception;
pub use exception::YoutubeError;

pub type AppResult<T> = Result<T, YoutubeError>;

mod extract;

mod helpers;
pub use helpers::*;

mod parser;
pub use parser::*;
