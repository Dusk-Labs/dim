use displaydoc::Display;
use thiserror::Error;

#[derive(Clone, Debug, Error, Display)]
pub enum Error {
    /// Abc
    Abc
}
