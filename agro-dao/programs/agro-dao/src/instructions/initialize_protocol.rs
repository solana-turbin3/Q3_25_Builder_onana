use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    /// The protocol state account to be initialized
    #[account(
        init,
        payer = authority,
        space = 8 + ProtocolState::INIT_SPACE,
        seeds = [b"protocol_state"],
        bump
    )]
    pub protocol_state: Account<'info, ProtocolState>,

    /// The authority that can update the protocol parameters
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProtocol<'info> {
    pub fn initialize_protocol(
        &mut self, bump: u8) -> Result<()> {
            self.protocol_state.set_inner(ProtocolState {
                authority: self.authority.key(),
                proposal_id_counter: 0,
                min_funding_threshold: 0,
                research_proposal_fee: 0,
                ipfs_hash_of_agri_data: [0; 32],
                research_data_counter: 0,
                creation_timestamp: Clock::get()?.unix_timestamp,
                minimum_staked_amount: 0,
                protocol_version: 1,
                is_paused: false,
                bump,
                reserved: [0; 128],
            });
     
        Ok(())
        }
}



