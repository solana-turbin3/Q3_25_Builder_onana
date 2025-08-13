use anchor_lang::prelude::*;
use crate::error::TreasuryError;

// Research DAO program interface for CPI
#[derive(Clone)]
pub struct ResearchDao;

impl anchor_lang::Id for ResearchDao {
    fn id() -> Pubkey {
        // Replace with actual research DAO program ID
        "7KvSwDEKz7KfMGbmNhQzZXLBmyGiSgbDqW8VK7Lz4wxd".parse().unwrap()
    }
}

// CPI function wrapper for validation
pub fn validate_proposal_for_funding_cpi<'info>(
    research_dao_program: AccountInfo<'info>,
    proposal: AccountInfo<'info>,
    researcher_profile: AccountInfo<'info>,
    proposal_id: u64,
    funding_amount: u64,
) -> Result<()> {
    // For now, just do basic validation
    // This would be replaced with actual CPI call to research DAO
    
    // Validate accounts are owned by research DAO
    require!(
        proposal.owner == &research_dao_program.key(),
        TreasuryError::Unauthorized
    );
    
    require!(
        researcher_profile.owner == &research_dao_program.key(),
        TreasuryError::Unauthorized
    );
    
    // Additional validation would go here via CPI
    // For now, we'll just ensure the accounts are valid
    
    msg!("Proposal {} validated for funding amount {}", proposal_id, funding_amount);
    
    Ok(())
}
