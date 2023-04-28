use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct InstructionThingArgs {
    value: u64,
}

#[derive(Debug, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum InstructionThingy {
    /// Description of this instruction
    #[account(0, writable, signer, name="signed_writable_account", desc="signed, writable account description")]
    #[account(1, writable, name="writable_account", desc = "writable, non signed account description")]
    #[account(2, name="non_writable_account", desc = "non signed, non writable account description")]
    #[account(3, name="token_program", desc = "Token program")]
    #[account(4, name="rent", desc = "Rent sysvar")]
    InstructionThing(InstructionThingArgs),
}
