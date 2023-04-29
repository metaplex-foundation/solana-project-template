use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum ErrorThingy {
    /// My error description
    #[error("My error message")]
    MyErrorName,
}

impl PrintProgramError for ErrorThingy {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<ErrorThingy> for ProgramError {
    fn from(e: ErrorThingy) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ErrorThingy {
    fn type_of() -> &'static str {
        "Error Thingy"
    }
}
