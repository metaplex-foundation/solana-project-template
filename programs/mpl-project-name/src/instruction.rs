use shank::{ShankContext, ShankInstruction};

use crate::processor::CreateArgs;

/// Instruction discriminants for routing.
/// The first byte of instruction data determines which instruction to execute.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MplProjectNameInstructionDiscriminant {
    Create = 0,
}

impl TryFrom<u8> for MplProjectNameInstructionDiscriminant {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MplProjectNameInstructionDiscriminant::Create),
            _ => Err(()),
        }
    }
}

/// Instruction enum for Shank IDL generation.
/// Note: We keep Shank attributes for IDL generation but use zero-copy
/// for actual instruction deserialization in the processor.
#[derive(Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum MplProjectNameInstruction {
    /// Create My Account.
    /// A detailed description of the instruction.
    #[account(0, writable, signer, name="address", desc = "The address of the new account")]
    #[account(1, name="authority", desc = "The authority of the new account")]
    #[account(2, writable, signer, name="payer", desc = "The account paying for the storage fees")]
    #[account(3, name="system_program", desc = "The system program")]
    Create(CreateArgs),
}
