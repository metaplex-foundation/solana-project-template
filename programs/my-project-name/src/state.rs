use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct AccountThingy {
    thing: u8,
}
