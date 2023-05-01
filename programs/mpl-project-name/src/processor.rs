use crate::error::MplProjectNameError;
use crate::instruction::{CreateMyAccountArgs, MplProjectNameInstruction};
use crate::state::MyAccount;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};

pub struct Processor;
impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: MplProjectNameInstruction =
            MplProjectNameInstruction::try_from_slice(instruction_data)?;
        match instruction {
            MplProjectNameInstruction::CreateMyAccount(args) => {
                msg!("Instruction: CreateMyAccount");
                create_my_account(accounts, args)
            }
        }
    }
}

fn create_my_account(accounts: &[AccountInfo], args: CreateMyAccountArgs) -> ProgramResult {
    // Accounts.
    let account_info_iter = &mut accounts.iter();
    let address = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent = Rent::get()?;

    // Guards.
    if *system_program.key != system_program::id() {
        return Err(MplProjectNameError::InvalidSystemProgram.into());
    }

    // Fetch the space and minimum lamports required for rent exemption.
    let space: usize = MyAccount::LEN;
    let lamports: u64 = rent.minimum_balance(space);

    // CPI to the System Program.
    invoke(
        &system_instruction::create_account(
            payer.key,
            address.key,
            lamports,
            space as u64,
            &crate::id(),
        ),
        &[payer.clone(), address.clone(), system_program.clone()],
    )?;

    // TODO: Assign the data.
    Ok(())
}
