use anchor_lang::{
    prelude::*, 
    system_program::{
        transfer, 
        Transfer
    }
};

use mpl_core::{
    instructions::UpdatePluginV1CpiBuilder, 
    types::{
        Attribute, 
        Attributes, 
        Plugin
    }
};

use crate::{
    constants::UserMilestone, 
    state::{
        Donations,
        Profile, 
        User, 
        VaultState
    }
};

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,
    #[account(mut)]
    pub profile: Account<'info, Profile>,
    #[account(
        init_if_needed,
        payer = donor,
        seeds = [b"donor", donor.key().as_ref(), profile.key().as_ref()],
        bump,
        space = User::INIT_SPACE,
    )]
    pub user_account: Account<'info, User>,

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

    /// CHECK: This account should be updated with the new NFT values
    pub core_nft_account: AccountInfo<'info>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

// TBD: Implement fee
// TBD: Implement reflection period of 72h (hold updates to user account and core NFT with the contributions) ?? Or this is done only at refund?
// TBD: Enable donating only within profile duration window!!
impl<'info> Donate<'info> {
    pub fn transfer_to_vault(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.donor.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn update_donation_track(&mut self, amount: u64) -> Result<()> {

        // if entry for the donor already exist -> update total amount donated
        if let Some(donation) = self.profile.donations_list.iter_mut().find(|donation| donation.donor == self.donor.key()) {
            donation.amount += amount;
        }
        // if new donor -> create new entry
        else {
            let new_donation = Donations {
                donor: self.donor.key(),
                amount,
            };
            
            self.profile.donations_list.push(new_donation);
        }

        Ok(())
    }

    pub fn update_user_account(&mut self, amount: u64, bumps: DonateBumps) -> Result<()> {

        self.user_account.set_inner( User { 
            owner: self.donor.key(), 
            profile: self.profile.key(), 
            amount_donated: self.user_account.amount_donated + amount,
            last_donation_date: Clock::get()?.unix_timestamp, 
            bump: bumps.user_account,
        });

        Ok(())
    }

    // TBD: Point system and badges definition
    pub fn update_profile_nft(&mut self, points: u64) -> Result<()> { // [AC] check for correctness and aplicability

       UpdatePluginV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.core_nft_account.to_account_info())
            .payer(&self.donor.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .plugin(Plugin::Attributes(Attributes { attribute_list: 
                vec![
                    Attribute {
                        key: "Bagde".to_string(),
                        value: UserMilestone::DonorNewbie.to_string(),  // TBD: Define update
                    },
                    Attribute { 
                        key: "Total amount donated".to_string(), 
                        value: self.user_account.amount_donated.to_string(), 
                    },
                    Attribute { 
                        key: "Points".to_string(),
                        value: points.to_string(), 
                    },
                ]
            }))
            .invoke()?;
            
        Ok(())
    }

}