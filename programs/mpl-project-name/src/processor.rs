use crate::instruction::MplProjectNameInstruction;
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: MplProjectNameInstruction =
            MplProjectNameInstruction::try_from_slice(instruction_data)?;
        match instruction {
            MplProjectNameInstruction::CreateMyAccount(_args) => {
                // handle instruction
                Ok(())
            }
        }
    }
}
