mod my_account;
mod my_data;
mod my_pda_account;

pub use my_account::*;
pub use my_data::*;
pub use my_pda_account::*;

use shank::ShankType;

/// Account discriminator enum.
/// Stored as a u8 in account data but represented as an enum for type safety.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, ShankType)]
pub enum Key {
    Uninitialized,
    MyAccount,
    MyPdaAccount,
}

impl From<u8> for Key {
    fn from(value: u8) -> Self {
        match value {
            0 => Key::Uninitialized,
            1 => Key::MyAccount,
            2 => Key::MyPdaAccount,
            _ => Key::Uninitialized,
        }
    }
}

impl From<Key> for u8 {
    fn from(value: Key) -> Self {
        value as u8
    }
}
