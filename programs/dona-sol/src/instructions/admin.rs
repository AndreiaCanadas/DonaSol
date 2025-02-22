use anchor_lang::prelude::*;

use crate::{
    constants::VerificationStatus, 
    errors::DonaSolError, 
    state::{
        Institution, 
        Profile, 
        Settings
    }
};

#[derive(Accounts)]
pub struct Admin<'info> {
    pub user: Signer<'info>,
    pub settings: Account<'info, Settings>,
    #[account(
        seeds = [b"institution", user.key().as_ref()],
        bump = institution.bump,
    )]
    pub institution: Option<Account<'info, Institution>>,
    #[account(
        seeds = [b"profile", user.key().as_ref()],
        bump = profile.bump,
    )]
    pub profile: Option<Account<'info, Profile>>,
}

impl<'info> Admin<'info> {
    pub fn set_status_institution(&mut self, status: VerificationStatus) -> Result<()> {

        require!(self.user.key() == self.settings.admin && self.institution.is_some(), DonaSolError::UserIsNotAdmin);

        self.institution.clone().unwrap().verification_status = status;

        Ok(())
    }

    pub fn set_status_profile(&mut self, status: VerificationStatus) -> Result<()> {

        require!(self.user.key() == self.settings.admin && self.profile.is_some(), DonaSolError::UserIsNotAdmin);

        self.profile.clone().unwrap().verification_status = status;

        Ok(())
    }
    
}

