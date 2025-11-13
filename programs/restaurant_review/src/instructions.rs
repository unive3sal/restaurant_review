use anchor_lang::prelude::*;

use crate::{
    state::*,
    errors::ReviewError,
};

#[derive(Accounts)]
#[instruction(content: ReviewInput)]
pub struct AddReviewContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // #[account(init)] will create the account via a CPI
    #[account(
        init,
        payer = signer,
        space = ReviewContent::INIT_SPACE,
        seeds = [b"review", signer.key().as_ref()],
        bump,
    )]
    pub review_content: Account<'info, ReviewContent>,

    pub system_program: Program<'info, System>,
}

pub fn add_review(ctx: Context<AddReviewContext>, content: ReviewInput) -> Result<()> {
    let review = &mut ctx.accounts.review_content;
    review.title = content.title;
    review.rating = content.rating;
    review.description = content.description;
    Ok(())
}

#[derive(Accounts)]
#[instruction(content: ReviewInput)]
pub struct UpdateReviewContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"review", signer.key().as_ref()],
        bump,
        constraint = existing_review.reviewer == signer.key()
    )]
    pub existing_review: Account<'info, ReviewContent>,
}

pub fn update_review(ctx: Context<UpdateReviewContext>, content: ReviewInput) -> Result<()> {
    let review = &mut ctx.accounts.existing_review;
    review.title = content.title;
    review.rating = content.rating;
    review.description = content.description;
    Ok(())
}

#[derive(Accounts)]
pub struct DeleteReviewContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"review", signer.key().as_ref()],
        bump,
        constraint = existing_review.reviewer == signer.key() @ ReviewError::ReviewOwnershipMismatch,
        close = signer,
    )]
    pub existing_review: Account<'info, ReviewContent>,
}

pub fn delete_review(_ctx: Context<DeleteReviewContext>) -> Result<()> {
    Ok(())
}
