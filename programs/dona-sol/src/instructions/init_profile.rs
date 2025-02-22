use anchor_lang::prelude::*;

use crate::{
    constants::{
        ProfileType, 
        VerificationStatus, 
        ANCHOR_DISCRIMINATOR
    }, 
    state::{
        Profile, 
        VaultState
    }
};

#[derive(Accounts)]
pub struct InitProfile<'info> {  // TBD: Add restriction for signer to be a pubkey that belongs to a verified institution
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"profile", owner.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR + Profile::INIT_SPACE,
    )]
    pub profile: Account<'info, Profile>,
    
    // TBD: vault as system account to receive SOL ? Only one vault per profile?
    // TBD: other vault to receive spl tokens ?
    #[account(
        init,
        payer = owner,
        seeds = [b"state", profile.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitProfile<'info> {
    pub fn init_profile(&mut self, target: u64, duration: u16, category: ProfileType, name: String, description: String, bumps: InitProfileBumps) -> Result<()> {
        self.profile.set_inner(Profile { 
            owner: self.owner.key(), 
            target, 
            start_date: Clock::get()?.unix_timestamp, 
            duration, 
            verification_status: VerificationStatus::Pending,
            bump: bumps.profile,
            category,
            name, 
            description,
            donations_list: Vec::new(),
        });
        self.vault_state.set_inner(VaultState {
            vault_bump: self.vault_state.vault_bump,
            state_bump: self.vault_state.state_bump,
        });
        Ok(())
    }
}