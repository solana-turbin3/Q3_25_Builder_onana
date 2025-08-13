use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;

/// CPI helper functions for other programs to call reputation program
pub struct ReputationCpi;

impl ReputationCpi {
    /// Helper function for other programs to update reputation via CPI
    pub fn update_reputation<'info>(
        reputation_program: AccountInfo<'info>,
        reputation_config: AccountInfo<'info>,
        user_reputation: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        user: Pubkey,
        event_type: ReputationEvent,
        custom_amount: Option<i64>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_accounts = crate::cpi::accounts::UpdateReputation {
            reputation_config,
            user_reputation,
            authority,
        };
        
        let cpi_context = CpiContext::new_with_signer(
            reputation_program,
            cpi_accounts,
            signer_seeds,
        );
        
        crate::cpi::update_reputation(cpi_context, user, event_type, custom_amount)
    }
    
    /// Helper function for other programs to decrease reputation on failure via CPI
    pub fn decrease_reputation_on_failure<'info>(
        reputation_program: AccountInfo<'info>,
        reputation_config: AccountInfo<'info>,
        user_reputation: AccountInfo<'info>,
        authority: AccountInfo<'info>,
        user: Pubkey,
        failure_type: crate::instructions::FailureType,
        custom_penalty: Option<i64>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_accounts = crate::cpi::accounts::DecreaseReputationOnFailure {
            reputation_config,
            user_reputation,
            authority,
        };
        
        let cpi_context = CpiContext::new_with_signer(
            reputation_program,
            cpi_accounts,
            signer_seeds,
        );
        
        crate::cpi::decrease_reputation_on_failure(cpi_context, user, failure_type, custom_penalty)
    }
    
    /// Helper function to get reputation data via CPI
    pub fn get_reputation<'info>(
        reputation_program: AccountInfo<'info>,
        reputation_config: AccountInfo<'info>,
        user_reputation: AccountInfo<'info>,
        user: Pubkey,
    ) -> Result<ReputationData> {
        let cpi_accounts = crate::cpi::accounts::GetReputation {
            reputation_config,
            user_reputation,
        };
        
        let cpi_context = CpiContext::new(
            reputation_program,
            cpi_accounts,
        );
        
        crate::cpi::get_reputation(cpi_context, user)
    }
}

/// Seeds helper for deriving reputation accounts
pub struct ReputationSeeds;

impl ReputationSeeds {
    pub fn reputation_config() -> &'static [u8] {
        REPUTATION_CONFIG_SEED
    }
    
    pub fn user_reputation(user: &Pubkey) -> [&[u8]; 2] {
        [USER_REPUTATION_SEED, user.as_ref()]
    }
}
