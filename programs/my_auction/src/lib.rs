use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, InitializeMint, initialize_mint};

declare_id!("Bbnt2AVTcEgGwhAShxjwSFT6dUaSgM8kmKhtp3kvvnAD");

mod errors;
use crate::errors::AuctionError;


#[program]
pub mod my_auction {
    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        let (_, bump_seed) = Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);
        let seeds = &[
            ctx.accounts.mint.to_account_info().key.as_ref(),
            &[bump_seed],
        ];

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let signer = &[&seeds[..]];
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        initialize_mint(cpi_context, 0, ctx.accounts.mint_authority.key, None);
        


        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(auction_duration: u64, initializer_amount: u64)]
pub struct Initialize<'info> {
    
    /// CHECK:
    #[account(
        init,
        payer = exhibitor,
        space = Mint::LEN,
        owner = anchor_spl::token::ID
    )]
    pub mint: AccountInfo<'info>,
    

    #[account(mut)]
    pub exhibitor: Signer<'info>,
    #[account(
        mut,
        constraint = exhibitor_nft_token_account.amount == 1
    )]
    pub exhibitor_nft_token_account: Account<'info, TokenAccount>,
    pub exhibitor_nft_temp_account: Account<'info, TokenAccount>,
    pub exhibitor_ft_receiving_account:Account<'info, TokenAccount>,
    

    // #[account(zero)]
    #[account(
        init,
        payer = exhibitor,
        space = 8 + std::mem::size_of::<Auction>()
    )]
    pub escrow_account: Box<Account<'info, Auction>>,

    /// CHECK:
    #[account(seeds = [mint.key().as_ref()], bump)]
    pub mint_authority: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}
impl<'info> Initialize<'info> {

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
