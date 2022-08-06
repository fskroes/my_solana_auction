use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, InitializeMint, initialize_mint, Burn, burn, Token, MintTo, mint_to};
use anchor_lang::solana_program::{program::invoke, system_program, system_instruction, native_token::sol_to_lamports};
use std::ops::Add;

declare_id!("Bbnt2AVTcEgGwhAShxjwSFT6dUaSgM8kmKhtp3kvvnAD");


#[program]
pub mod my_auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, auction_duration: u64, initial_price: u64) -> Result<()> {
        
        let result = initialize_mint(
            ctx.accounts.init_context_mint(), 
            0, 
            ctx.accounts.mint_authority.key, 
            None
        );

        match result {
            Ok(_) => msg!("Mint is initialized. PDA is created"),
            Err(error_message) => msg!("Something went wrong with initialising mint {:#?}", error_message.log()),
        }
        
        let auction = &mut ctx.accounts.escrow_account;

        auction.exhibitor_pubkey = ctx.accounts.exhibitor.key();
        auction.exhibitor_ft_receiving_pubkey = ctx.accounts.exhibitor_ft_receiving_account.key();
        auction.exhibiting_nft_temp_pubkey = ctx.accounts.exhibitor_nft_temp_account.key();
        auction.highest_bidder_pubkey = ctx.accounts.exhibitor.key();
        auction.highest_bidder_ft_temp_pubkey = ctx.accounts.exhibitor_ft_receiving_account.key();
        auction.highest_bidder_ft_returning_pubkey = ctx.accounts.exhibitor_ft_receiving_account.key();
        auction.price = initial_price;
        auction.end_at = ctx.accounts.clock.unix_timestamp.add(auction_duration as i64);

        auction.bump = *ctx.bumps.get("mint_authority").unwrap();
        auction.mint = ctx.accounts.mint.key();
        auction.treasury = ctx.accounts.treasury.key();

        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, price: u64) -> Result<()> {

        invoke(
            &system_instruction::transfer(
                ctx.accounts.bidder.key, 
                ctx.accounts.treasury.key, 
                sol_to_lamports(price as f64)
            ),
            &[
                ctx.accounts.bidder.to_account_info().clone(),
                ctx.accounts.treasury.clone(),
            ],
        )?;

        let auction = &mut ctx.accounts.escrow_account;
        auction.price = price;

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info().clone(),
            to: ctx.accounts.bidder_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };

        let seeds = &[
            ctx.accounts.mint.to_account_info().key.as_ref(),
            &[ctx.accounts.escrow_account.bump],
        ];
        let signer = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        mint_to(cpi_ctx, price)?;

        // burn(ctx.accounts.init_burn_context(), price)?;

        


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

    /// CHECK: 
    #[account(mut, signer)]
    pub exhibitor: AccountInfo<'info>,

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

    /// CHECK:
    pub treasury: AccountInfo<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
    
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}
impl<'info> Initialize<'info> {
    pub fn init_context_mint(&self) -> CpiContext<'_, '_, '_, 'info, InitializeMint<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = InitializeMint {
            mint: self.mint.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

#[derive(Accounts)]
#[instruction(price: u64)]
pub struct Bid<'info> {
    /// CHECK:
    #[account(mut, signer)]
    pub bidder: AccountInfo<'info>,

    // #[account(mut)]
    // pub bidder_ft_temp_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub bidder_token_account: Account<'info, TokenAccount>,

    #[account(mut, has_one = mint, has_one = treasury)]
    pub escrow_account: Account<'info, Auction>,

    /// CHECK:
    #[account(seeds = [mint.key().as_ref()], bump = escrow_account.bump)]
    pub mint_authority: AccountInfo<'info>,
    
    /// CHECK:
    #[account(mut)]
    pub treasury: AccountInfo<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,
    /// CHECK:
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,

}
impl<'info> Bid<'info> {
    pub fn init_burn_context(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.mint.to_account_info(),
            authority: self.bidder.to_account_info(),
            from: self.bidder_token_account.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
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

    pub mint: Pubkey,
    pub treasury: Pubkey,
    pub bump: u8,
}