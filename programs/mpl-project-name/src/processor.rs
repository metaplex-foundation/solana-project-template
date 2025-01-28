use borsh::BorshDeserialize;
use bytemuck::from_bytes_mut;
use pinocchio::account_info::AccountInfo;
use pinocchio::pubkey::Pubkey;
use pinocchio::sysvars::rent::Rent;
use pinocchio::sysvars::Sysvar;
use pinocchio::ProgramResult;

use crate::error::MplProjectNameError;
use crate::instruction::accounts::CreateAccounts;
use crate::instruction::{CreateArgs, MplProjectNameInstruction};
use crate::state::MyAccount;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: MplProjectNameInstruction =
        MplProjectNameInstruction::try_from_slice(instruction_data)
            .map_err(|_| MplProjectNameError::InvalidInstruction)?;
    match instruction {
        MplProjectNameInstruction::Create(args) => {
            pinocchio::log::sol_log("Instruction: Create");
            create(accounts, args)
        }
    }
}

fn create(accounts: &[AccountInfo], _args: CreateArgs) -> ProgramResult {
    // Accounts.
    let ctx = CreateAccounts::context(accounts)?;
    let rent = Rent::get()?;

    // Guards.
    if *ctx.accounts.system_program.key() != pinocchio_system::id() {
        return Err(MplProjectNameError::InvalidSystemProgram.into());
    }

    // Fetch the space and minimum lamports required for rent exemption.
    let space: usize = MyAccount::LEN;
    let lamports: u64 = rent.minimum_balance(space);

    // CPI to the System Program.
    pinocchio_system::instructions::CreateAccount {
        from: ctx.accounts.payer,
        to: ctx.accounts.address,
        lamports,
        space: space as u64,
        owner: &crate::ID,
    }
    .invoke()?;

    let mut my_account_data = ctx.accounts.address.try_borrow_mut_data()?;
    let my_account: &mut MyAccount = from_bytes_mut(&mut my_account_data[..]);

    my_account.discriminator = [1; 8];
    my_account.authority = *ctx.accounts.authority.key();
    my_account.data = [2; 32];

    Ok(())
}
