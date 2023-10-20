//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Key;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MyPdaAccount {
    pub key: Key,
    pub bump: u8,
}

impl MyPdaAccount {
    pub const LEN: usize = 2;

    pub fn create_pda(
        authority: Pubkey,
        name: &str,
        bump: u8,
    ) -> Result<solana_program::pubkey::Pubkey, solana_program::pubkey::PubkeyError> {
        solana_program::pubkey::Pubkey::create_program_address(
            &[
                "myPdaAccount".as_bytes(),
                crate::MPL_PROJECT_NAME_PROGRAM_ID.as_ref(),
                authority.as_ref(),
                name.to_string().as_ref(),
                &[bump],
            ],
            &crate::MPL_PROJECT_NAME_PROGRAM_ID,
        )
    }

    pub fn find_pda(authority: &Pubkey, name: &str) -> (solana_program::pubkey::Pubkey, u8) {
        solana_program::pubkey::Pubkey::find_program_address(
            &[
                "myPdaAccount".as_bytes(),
                crate::MPL_PROJECT_NAME_PROGRAM_ID.as_ref(),
                authority.as_ref(),
                name.to_string().as_ref(),
            ],
            &crate::MPL_PROJECT_NAME_PROGRAM_ID,
        )
    }

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for MyPdaAccount {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}
