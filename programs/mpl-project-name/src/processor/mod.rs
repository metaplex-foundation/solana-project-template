mod create;

use bytemuck::try_from_bytes;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::error::MplProjectNameError;
use crate::instruction::MplProjectNameInstructionDiscriminant;

pub use create::*;

/// Process incoming instructions.
///
/// # Arguments
/// * `_program_id` - The program ID (unused but available for validation)
/// * `accounts` - The accounts required for the instruction
/// * `instruction_data` - The instruction data containing the discriminant and arguments
#[inline]
pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    // Ensure we have at least 1 byte for the discriminant.
    if instruction_data.is_empty() {
        return Err(MplProjectNameError::InvalidInstructionData.into());
    }

    // Route by discriminant (first byte).
    match MplProjectNameInstructionDiscriminant::try_from(instruction_data[0]) {
        Ok(MplProjectNameInstructionDiscriminant::Create) => {
            msg!("Instruction: Create");
            create(
                accounts,
                try_from_bytes(instruction_data)
                    .map_err(|_| MplProjectNameError::InvalidInstructionData)?,
            )
        }
        Err(_) => Err(MplProjectNameError::InvalidInstructionData.into()),
    }
}
