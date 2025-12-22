use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Key, MyData};

/// Main account structure using zero-copy patterns.
///
/// # Layout
/// - key: 1 byte (account discriminator)
/// - _padding: 7 bytes (alignment to 8 bytes)
/// - authority: 32 bytes
/// - data: 8 bytes (MyData)
///
/// Total: 48 bytes (8-byte aligned)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable, ShankAccount)]
pub struct MyAccount {
    /// Account discriminator.
    #[idl_type(Key)]
    pub key: u8,
    /// Padding for 8-byte alignment.
    pub _padding: [u8; 7],
    /// The authority of this account.
    pub authority: Pubkey,
    /// Account-specific data.
    pub data: MyData,
}

// Compile-time assertion to ensure struct is 8-byte aligned.
const _: () = assert!(core::mem::size_of::<MyAccount>() % 8 == 0);
const _: () = assert!(core::mem::size_of::<MyAccount>() == 48);

impl MyAccount {
    /// The base length of the account in bytes.
    pub const BASE_LEN: usize = core::mem::size_of::<MyAccount>();

    /// PDA seed prefix for this account type (if used as a PDA).
    pub const PREFIX: &'static [u8] = b"my_account";

    /// Initialize the account with the given values.
    #[inline]
    pub fn initialize(&mut self, authority: Pubkey, data: MyData) {
        self.key = Key::MyAccount as u8;
        self._padding = [0u8; 7];
        self.authority = authority;
        self.data = data;
    }
}
