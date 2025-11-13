use anchor_lang::prelude::*;

#[error_code]
pub enum ReviewError {
    #[msg("user is not the owner of current review")]
    ReviewOwnershipMismatch,
}
