use anchor_lang::prelude::*;

declare_id!("8dDR8ExmWPvi4k3UYDT1Vr47c1Z5MVC1CrWfT3KH6KMt");

mod state;
mod instructions;
mod errors;
mod constants;

use state::*;
use instructions::*;
use constants::*;

#[program]
pub mod dona_sol {

    use super::*;

    pub fn init_settings(ctx: Context<InitSettings>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps)?;

        Ok(())
    }

    pub fn set_settings(ctx: Context<SetSettings>, new_admin: Pubkey) -> Result<()> {
        ctx.accounts.set_settings(new_admin)?;

        Ok(())
    }

    pub fn init_institution(ctx: Context<InitInstitution>, name: String, website: String) -> Result<()> {
        ctx.accounts.init_institution(name, website, ctx.bumps)?;

        Ok(())
    }

    pub fn init_profile(ctx: Context<InitProfile>, target: u64, duration: u16, category: ProfileType, name: String, description: String) -> Result<()> {
        ctx.accounts.init_profile(target, duration, category, name, description, ctx.bumps)?;

        Ok(())
    }

    pub fn set_status_institution(ctx: Context<Admin>, status: VerificationStatus) -> Result<()> {
        ctx.accounts.set_status_institution(status)?;

        Ok(())
    }

    pub fn set_status_profile(ctx: Context<Admin>, status: VerificationStatus) -> Result<()> {
        ctx.accounts.set_status_profile(status)?;

        Ok(())
    }

    pub fn init_user(ctx: Context<InitUser>, name: String) -> Result<()> {
        ctx.accounts.init_user_account(ctx.bumps)?;
        ctx.accounts.mint_profile_nft(name)?;

        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_to_vault(amount)?;
        ctx.accounts.update_user_account(amount, ctx.bumps)?;
        ctx.accounts.update_profile_nft(amount*10)?;               // TBD: points roadmap

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_donor()?;

        Ok(())
    }

    pub fn transfer_to_institution(ctx: Context<WithdrawFunds>) -> Result<()> {

        ctx.accounts.transfer_to_institution();
        
        //ctx.remaining_accounts

        Ok(())
    }

}

