pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GSBPFqwx4FphgCBEQkf8o3QS5LbtQrjo8RjPq18bqwWg");

#[program]
pub mod anchor_dice {
    use super::*;


}
