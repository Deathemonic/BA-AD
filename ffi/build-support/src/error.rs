use std::env::VarError;
use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Environment(#[from] VarError),
    #[error(transparent)]
    Io(#[from] io::Error)
}
