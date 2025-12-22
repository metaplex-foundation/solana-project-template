use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MplProjectNameError {
    /// 0 - Invalid System Program
    #[error("Invalid System Program")]
    InvalidSystemProgram,
    /// 1 - Invalid instruction data
    #[error("Invalid instruction data")]
    InvalidInstructionData,
    /// 2 - Invalid account data
    #[error("Invalid account data")]
    InvalidAccountData,
}

impl PrintProgramError for MplProjectNameError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MplProjectNameError> for ProgramError {
    fn from(e: MplProjectNameError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MplProjectNameError {
    fn type_of() -> &'static str {
        "Mpl Project Name Error"
    }
}
