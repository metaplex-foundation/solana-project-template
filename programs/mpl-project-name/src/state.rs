use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub enum Key {
    Uninitialized,
    MyAccount,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, Debug, ShankAccount)]
pub struct MyAccount {
    key: Key,
    thing: u8,
    defined_type: MyDefinedType,
}

#[derive(Clone, BorshSerialize, BorshDeserialize, Debug)]
pub struct MyDefinedType {
    another_thing: u8,
}
