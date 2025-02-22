use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{errors::DonaSolError, state::{
    Institution,
    Profile
}};

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub institution: Signer<'info>,
    #[account(
        seeds = [b"institution", institution.key().as_ref()],
        bump,
    )]
    pub institution_account: Account<'info, Institution>,
    #[account(
        mut,
        seeds = [b"profile", institution.key().as_ref()],
        bump,
    )]
    pub profile: Account<'info, Profile>,

    // TBD: vault definition
    #[account(
        seeds = [b"state", profile.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFunds<'info> {
    pub fn transfer_to_institution(&mut self) -> Result<()> {

        require_eq!(self.institution.key(), self.profile.owner.key(), DonaSolError::SignerIsNotOwner);

        let vault_amount = self.vault.lamports();

        // If project campaign deadline has not been reached
        if Clock::get()?.unix_timestamp >= self.profile.start_date + self.profile.duration as i64 {
            return err!(DonaSolError::DeadlineNotReached);
        }
        // If project target has not been reached
        if vault_amount < self.profile.target {
            return err!(DonaSolError::TargetNotReached);
        }

        let vault_state_key = self.vault_state.key();
        let seeds = &[
            b"vault",
            vault_state_key.as_ref(),
            &[self.vault_state.state_bump]
        ];     
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.institution.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, vault_amount)?;

        Ok(())
    }
}
