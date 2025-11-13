use anchor_lang::prelude::*;

mod instructions;
mod state;
mod errors;

declare_id!("6GvL2zxnF2pU1QsDSUkvxwnXF4uNGg12TiPS1Lg3NJiv");

#[program]
pub mod restaurant_review {
    pub use super::state::*;
    pub use super::instructions::*;
    use super::*;

    pub fn add_review(ctx: Context<AddReviewContext>, content: ReviewInput) -> Result<()> {
        instructions::add_review(ctx, content)
    }

    pub fn update_review(ctx: Context<UpdateReviewContext>, content: ReviewInput) -> Result<()> {
        instructions::update_review(ctx, content)
    }

    pub fn delete_review(ctx: Context<DeleteReviewContext>) -> Result<()> {
        instructions::delete_review(ctx)
    }
}
