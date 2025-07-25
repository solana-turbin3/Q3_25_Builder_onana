use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet{
    pub amount: u64,
    pub player: Pubkey, 
    pub slot: u64,
    pub seed: u128,
    pub roll: u8,
    pub bump: u8,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        let mut s: Vec<u8> = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&[self.roll, self.bump]);
        s
    }

}