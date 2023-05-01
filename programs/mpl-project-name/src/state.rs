use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum Key {
    Uninitialized,
    MyAccount,
    MyPdaAccount,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MyAccount {
    key: Key,
    authority: Pubkey,
    data: MyData,
}

impl MyAccount {
    pub const LEN: usize = 1 + 32 + MyData::LEN;
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MyPdaAccount {
    key: Key,
    bump: u8,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct MyData {
    version: u8,
    name: String,
}

impl MyData {
    pub const LEN: usize = 1 + 32;
}
