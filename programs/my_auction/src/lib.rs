use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(auction_duration: u64, initializer_amount: u64)]
pub struct Initialize<'info> {
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub exhibitor: AccountInfo<'info>,
    #[account(
        mut,
        constraint = exhibitor_nft_token_account.amount == 1
    )]
    pub exhibitor_nft_token_account: Account<'info, TokenAccount>,
    pub exhibitor_nft_temp_account: Account<'info, TokenAccount>,
    pub exhibitor_ft_receiving_account:Account<'info, TokenAccount>,
    
    #[account(zero)]
    pub escrow_account: Box<Account<'info, Auction>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

#[account]
pub struct Auction {

    pub exhibitor_pubkey: Pubkey,
    pub exhibiting_nft_temp_pubkey: Pubkey,
    pub exhibitor_ft_receiving_pubkey: Pubkey,
    
    pub highest_bidder_pubkey: Pubkey,
    pub highest_bidder_ft_temp_pubkey: Pubkey,
    pub highest_bidder_ft_returning_pubkey: Pubkey,

    pub price: u64,
    pub end_at: i64,

}
