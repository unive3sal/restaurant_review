use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ReviewContent {
    pub reviewer: Pubkey,
    #[max_len(50)]
    pub title: String,
    pub rating: u8,
    #[max_len(256)]
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ReviewInput {
    pub title: String,
    pub rating: u8,
    pub description: String,
}
