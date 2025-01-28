use num_derive::FromPrimitive;
use pinocchio::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum MplProjectNameError {
    /// 0 - Invalid System Program
    #[error("Invalid System Program")]
    InvalidSystemProgram,
    /// 1 - Error deserializing account
    #[error("Error deserializing account")]
    DeserializationError,
    /// 2 - Error serializing account
    #[error("Error serializing account")]
    SerializationError,
    /// 3 - Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
}

impl From<MplProjectNameError> for ProgramError {
    fn from(e: MplProjectNameError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
