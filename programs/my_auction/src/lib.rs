use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::UnixTimestamp;

declare_id!("Poo5jhFcGjMjYaz2cpmSNVq4ehvjKJhjU7aCZiS2LMP");


#[program]
pub mod my_auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, duration: u64, amount: u64) -> Result<()> {
        
        let state = &mut ctx.accounts.auction_account;

        state.end_at = Clock::get()?.unix_timestamp + duration as UnixTimestamp;

        if amount == 0 {
            msg!("Initial price is 0");
            panic!("Initial price is 0");
        }

        state.treasury = *ctx.accounts.treasury.key;
        state.exhibitor_pubkey = *ctx.accounts.exhibitor.key;
        state.highest_bidder_pubkey = *ctx.accounts.exhibitor.key;
        state.price = amount;

        Ok(())
    }

    // pub fn bid(ctx: Context<Bid>, price: u64) -> Result<()> {

    //     invoke(
    //         &system_instruction::transfer(
    //             &ctx.accounts.bidder.key(), 
    //             &ctx.accounts.treasury.key(), 
    //             sol_to_lamports(price as f64)
    //         ),
    //         &[
    //             ctx.accounts.bidder.to_account_info().clone(),
    //             ctx.accounts.treasury.clone(),
    //         ],
    //     )?;

    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_accounts = MintTo {
    //         mint: ctx.accounts.mint.to_account_info().clone(),
    //         to: ctx.accounts.bidder_token_account.to_account_info(),
    //         authority: ctx.accounts.mint_authority.to_account_info(),
    //     };

    //     let seeds = &[
    //         ctx.accounts.mint.to_account_info().key.as_ref(),
    //         &[ctx.accounts.escrow_account.bump],
    //     ];
    //     let signer = &[&seeds[..]];
    //     let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    //     mint_to(cpi_ctx, price)?;

    //     let auction = &mut ctx.accounts.escrow_account;
    //     auction.price = price;
    //     auction.highest_bidder_pubkey = ctx.accounts.bidder.key();
    //     auction.highest_bidder_ft_returning_pubkey = ctx.accounts.bidder_token_account.key();


    //     Ok(())
    // }

    // pub fn refund(ctx: Context<Refund>) -> Result<()> {
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_accounts = MintTo {
    //         mint: ctx.accounts.mint.to_account_info().clone(),
    //         to: ctx.accounts.bidder_token_account.to_account_info(),
    //         authority: ctx.accounts.mint_authority.to_account_info(),
    //     };

    //     let seeds = &[
    //         ctx.accounts.mint.to_account_info().key.as_ref(),
    //         &[ctx.accounts.escrow_account.bump],
    //     ];
    //     let signer = &[&seeds[..]];
    //     let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
    //     // TODO there is a flaw in this design. Currently sending current amount of what is in the account.
    //     mint_to(cpi_ctx, ctx.accounts.bidder_token_account.amount)?;

    //     Ok(())
    // }

    
}

#[derive(Accounts)]
#[instruction(duration: u64, amount: u64)]
pub struct Initialize<'info> {

    /// CHECK:
    #[account(mut)]
    pub exhibitor: Signer<'info>,
    
    /// State of our auction program (up to you)
    #[account(
        init,
        payer = exhibitor,
        space = 8 + std::mem::size_of::<Auction>()
    )]
    pub auction_account: Account<'info, Auction>,

    /// Account which holds tokens bidded by biders
    #[account(
        init,
        payer = exhibitor,
        space = 0,
    )]
    /// CHECK:
    pub treasury: AccountInfo<'info>,
    /// CHECK: 
    pub system_program: Program<'info, System>,
}





#[account]
pub struct Auction {

    pub exhibitor_pubkey: Pubkey,
    pub highest_bidder_pubkey: Pubkey,

    pub price: u64,
    pub end_at: i64,

    pub treasury: Pubkey,
}