use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;
use shank::ShankAccount;

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Pod, Zeroable, Debug, ShankAccount, Copy)]
pub struct MyAccount {
    pub discriminator: [u8; 8],
    pub authority: Pubkey,
    pub data: [u8; 32],
}

impl MyAccount {
    pub const LEN: usize = 8 + 32 + 32;
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MyPdaAccount {
    pub discriminator: [u8; 8],
    pub bump: u8,
    pub padding: [u8; 7],
}
