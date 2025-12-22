use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

use super::Key;

/// PDA account structure using zero-copy patterns.
///
/// # Layout
/// - key: 1 byte (account discriminator)
/// - bump: 1 byte (PDA bump seed)
/// - _padding: 6 bytes (alignment to 8 bytes)
///
/// Total: 8 bytes (8-byte aligned)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable, ShankAccount)]
pub struct MyPdaAccount {
    /// Account discriminator.
    #[idl_type(Key)]
    pub key: u8,
    /// PDA bump seed.
    pub bump: u8,
    /// Padding for 8-byte alignment.
    pub _padding: [u8; 6],
}

// Compile-time assertion to ensure struct is 8-byte aligned.
const _: () = assert!(core::mem::size_of::<MyPdaAccount>() % 8 == 0);
const _: () = assert!(core::mem::size_of::<MyPdaAccount>() == 8);

impl MyPdaAccount {
    /// The base length of the account in bytes.
    pub const BASE_LEN: usize = core::mem::size_of::<MyPdaAccount>();

    /// PDA seed prefix for this account type.
    pub const PREFIX: &'static [u8] = b"my_pda_account";

    /// Initialize the account with the given bump seed.
    #[inline]
    pub fn initialize(&mut self, bump: u8) {
        self.key = Key::MyPdaAccount as u8;
        self.bump = bump;
        self._padding = [0u8; 6];
    }
}
