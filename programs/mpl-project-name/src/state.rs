use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::account_info::AccountInfo;
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
    pub key: Key,
    pub authority: Pubkey,
    pub data: MyData,
}

impl MyAccount {
    pub const LEN: usize = 1 + 32 + MyData::LEN;

    pub fn load(account: &AccountInfo) -> Self {
        let mut bytes: &[u8] = &(*account.data).borrow();
        MyAccount::deserialize(&mut bytes).unwrap()
    }

    pub fn save(&self, account: &AccountInfo) -> () {
        let mut bytes = Vec::with_capacity(account.data_len());
        self.serialize(&mut bytes).unwrap();
        account.try_borrow_mut_data().unwrap()[..bytes.len()].copy_from_slice(&bytes)
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MyPdaAccount {
    pub key: Key,
    pub bump: u8,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct MyData {
    pub foo: u16,
    pub bar: u32,
}

impl MyData {
    pub const LEN: usize = 2 + 4;
}
