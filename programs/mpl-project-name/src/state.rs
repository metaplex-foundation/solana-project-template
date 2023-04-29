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
    data: MyDefinedType,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MyPdaAccount {
    key: Key,
    bump: u8,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct MyDefinedType {
    thing: u8,
    hash: [u8; 32],
}
