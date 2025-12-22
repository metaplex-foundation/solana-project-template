use bytemuck::{Pod, Zeroable};
use shank::ShankType;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, rent::Rent,
    system_instruction, system_program, sysvar::Sysvar,
};

use crate::error::MplProjectNameError;
use crate::instruction::accounts::CreateAccounts;
use crate::state::{MyAccount, MyData};

/// Arguments for the Create instruction.
///
/// # Layout
/// - discriminator: 1 byte (instruction discriminant, excluded from IDL)
/// - _padding: 1 byte (alignment)
/// - arg1: 2 bytes
/// - arg2: 4 bytes
///
/// Total: 8 bytes (8-byte aligned)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable, ShankType)]
pub struct CreateArgs {
    /// Instruction discriminator (not included in IDL).
    #[skip]
    pub discriminator: u8,
    /// Padding for alignment.
    pub _padding: [u8; 1],
    /// Some description for arg1.
    pub arg1: u16,
    /// Some description for arg2.
    pub arg2: u32,
}

// Compile-time assertion to ensure struct is properly sized.
const _: () = assert!(core::mem::size_of::<CreateArgs>() == 8);

/// Create a new MyAccount.
///
/// # Accounts
/// 0. `[writable, signer]` address - The address of the new account
/// 1. `[]` authority - The authority of the new account
/// 2. `[writable, signer]` payer - The account paying for the storage fees
/// 3. `[]` system_program - The system program
///
/// # Arguments
/// * `accounts` - The accounts required for the instruction
/// * `args` - The instruction arguments (zero-copy reference)
pub fn create<'a>(accounts: &'a [AccountInfo<'a>], args: &CreateArgs) -> ProgramResult {
    /****************************************************/
    /****************** Account Setup *******************/
    /****************************************************/

    let ctx = CreateAccounts::context(accounts)?;
    let rent = Rent::get()?;

    /****************************************************/
    /****************** Account Guards ******************/
    /****************************************************/

    // Validate system program.
    if *ctx.accounts.system_program.key != system_program::id() {
        return Err(MplProjectNameError::InvalidSystemProgram.into());
    }

    /****************************************************/
    /***************** Argument Guards ******************/
    /****************************************************/

    // Add any argument validation here.
    // Example: if args.arg1 == 0 { return Err(MplProjectNameError::InvalidArgument.into()); }

    /****************************************************/
    /********************* Actions **********************/
    /****************************************************/

    // Fetch the space and minimum lamports required for rent exemption.
    let space: usize = MyAccount::BASE_LEN;
    let lamports: u64 = rent.minimum_balance(space);

    // CPI to the System Program to create the account.
    invoke(
        &system_instruction::create_account(
            ctx.accounts.payer.key,
            ctx.accounts.address.key,
            lamports,
            space as u64,
            &crate::id(),
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.address.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    // Initialize the account using zero-copy.
    // Borrow the account data mutably and cast to our struct.
    let mut data = ctx.accounts.address.try_borrow_mut_data()?;
    let my_account: &mut MyAccount = bytemuck::from_bytes_mut(&mut data[..MyAccount::BASE_LEN]);

    my_account.initialize(
        *ctx.accounts.authority.key,
        MyData::new(args.arg1, args.arg2),
    );

    Ok(())
}
