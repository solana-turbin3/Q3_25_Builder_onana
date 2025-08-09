use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct UpdateProtocol<'info> {
    /// The protocol state account to be updated
    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump,
        has_one = authority @ ErrorCode::UnauthorizedUpdate
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    /// The authority that can update the protocol parameters
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UpdateProtocolParams {
    pub min_funding_threshold: Option<u64>,
    pub research_proposal_fee: Option<u64>,
    pub minimum_staked_amount: Option<u64>,
    pub is_paused: Option<bool>,
    pub new_authority: Option<Pubkey>,
}

impl<'info> UpdateProtocol<'info> {
    pub fn update_protocol(
        &mut self,
        params: UpdateProtocolParams,
    ) -> Result<()> {
        // Verify protocol is not paused (unless we're unpausing it)
        if self.protocol_state.is_paused && params.is_paused != Some(false) {
            return Err(ErrorCode::ProtocolPaused.into());
        }

        // Check if minimum funding threshold is same as current
        if let Some(new_threshold) = params.min_funding_threshold {
            require!(
                new_threshold != self.protocol_state.min_funding_threshold,
                ErrorCode::SameValueUpdate
            );
        }

        // Check if research proposal fee is same as current
        if let Some(new_fee) = params.research_proposal_fee {
            require!(
                new_fee != self.protocol_state.research_proposal_fee,
                ErrorCode::SameValueUpdate
            );
        }

        //Check if minimum staked amount is same as current
        if let Some(new_stake_amount) = params.minimum_staked_amount {
            require!(
                new_stake_amount != self.protocol_state.minimum_staked_amount,
                ErrorCode::SameValueUpdate
            );
        }

        // Store current version for version increment
        let current_version = self.protocol_state.protocol_version;

        // Apply updates after all validations pass
        if let Some(threshold) = params.min_funding_threshold {
            self.protocol_state.min_funding_threshold = threshold;
            emit!(ParameterUpdated {
                parameter_name: "min_funding_threshold".to_string(),
                old_value: self.protocol_state.min_funding_threshold.to_string(),
                new_value: threshold.to_string(),
                authority: self.authority.key(),
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        if let Some(fee) = params.research_proposal_fee {
            self.protocol_state.research_proposal_fee = fee;
            emit!(ParameterUpdated {
                parameter_name: "research_proposal_fee".to_string(),
                old_value: self.protocol_state.research_proposal_fee.to_string(),
                new_value: fee.to_string(),
                authority: self.authority.key(),
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        if let Some(stake_amount) = params.minimum_staked_amount {
            self.protocol_state.minimum_staked_amount = stake_amount;
            emit!(ParameterUpdated {
                parameter_name: "minimum_staked_amount".to_string(),
                old_value: self.protocol_state.minimum_staked_amount.to_string(),
                new_value: stake_amount.to_string(),
                authority: self.authority.key(),
                timestamp: Clock::get()?.unix_timestamp,
            });
        }
        if let Some(paused) = params.is_paused {
            self.protocol_state.is_paused = paused;
            emit!(ProtocolPauseChanged {
                is_paused: paused,
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        if let Some(new_authority) = params.new_authority {
            let old_authority = self.protocol_state.authority;
            self.protocol_state.authority = new_authority;
            emit!(AuthorityChanged {
                old_authority,
                new_authority,
                timestamp: Clock::get()?.unix_timestamp,
            });
        }

        // Update protocol version to prevent replay attacks
        self.protocol_state.protocol_version = current_version.saturating_add(1);

        // Emit general update event
        emit!(ProtocolUpdated {
            authority: self.authority.key(),
            version: self.protocol_state.protocol_version,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

// Events for transparency and monitoring
#[event]
pub struct ProtocolUpdated {
    pub authority: Pubkey,
    pub version: u8,
    pub timestamp: i64,
}

#[event]
pub struct AuthorityChanged {
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct ProtocolPauseChanged {
    pub is_paused: bool,
    pub timestamp: i64,
}

#[event]
pub struct ParameterUpdated {
    pub parameter_name: String,
    pub old_value: String,
    pub new_value: String,
    pub authority: Pubkey,
    pub timestamp: i64,
}