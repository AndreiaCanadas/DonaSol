use anchor_lang::prelude::*;

use crate::constants::{ProfileType, VerificationStatus};


#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub owner: Pubkey,
    pub target: u64,                // target amount to collect in SOL
    pub start_date: i64,            // timestamp of project creation date
    pub duration: u16,              // fundraising duration in days
    pub verification_status: VerificationStatus,  // verification status: updated by platform admin
    pub bump: u8,
    pub category: ProfileType,           // type of project
    #[max_len(30)]
    pub name: String,               // project name
    #[max_len(50)]
    pub description: String,        // project description

    // TBD: implement a way to track user donations info (separated struct or included in Profile struct ?)
    // vector of tuples with pair (pubkey, total_amount) -> for refund purposes
    // [AC] Type tuple not supported in anchor?? -> Created struct -> Makes sense??
    // how to declare it? How to handle vector size?
    #[max_len(255)]
    pub donations_list: Vec<Donations>,        // track all donations (donor pubkey and amount)
}

// [AC] Should this be declared here? Does this make sense??
#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 1 + 1;
}

// [AC] Should this be declared here?
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Donations {
    pub donor: Pubkey,
    pub amount: u64,
}
// Space needs anchor discriminator??
impl Space for Donations {
    const INIT_SPACE: usize = 32 + 8;
}

