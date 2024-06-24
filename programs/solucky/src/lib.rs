use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Token, TokenAccount, Transfer},
};

use std::clone::Clone;
use solana_program::pubkey::Pubkey;

pub mod utils;
use utils::*;

pub const GLOBAL_AUTHORITY_SEED: &str = "global-authority";
pub const ADMIN_WALLET: &str = "ArZrqyPdd8YsBD67anP1fzbuwTCUfGCkofDutQYXp5Kc";

declare_id!("DuYWUDbqA2fwExk7jf6qYXQTndTvUcfwFjdfnFczK39i");

#[program]
pub mod presale_contract {
    use super::*;

    /**
     * @dev Initialize the project
     */
     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let global_authority = &mut ctx.accounts.global_authority;
        global_authority.admin = ctx.accounts.admin.key();
        Ok(())
    }

     pub fn create_presale(
        ctx: Context<CreatePresale>,
        min_allocation: u64,
        max_allocation: u64,
        hardcap: u64,
        softcap: u64,
        sale_price: u64,
        launch_price: u64,
        start_time: i64,
        end_time: i64,
        max_contribution: u64,
        base_decimals: u64,
        base_mint: Pubkey,
        quote_mint: Pubkey,
    ) -> Result<()> {
        let mut presale = ctx.accounts.presale.load_init()?;

        let src_token_account_info = &mut &ctx.accounts.creater_token_account;
        let dest_token_account_info = &mut &ctx.accounts.dest_token_account;
        let token_program = &mut &ctx.accounts.token_program;

        let cpi_accounts = Transfer {
            from: src_token_account_info.to_account_info().clone(),
            to: dest_token_account_info.to_account_info().clone(),
            authority: ctx.accounts.owner.to_account_info().clone(),
        }; 
        token::transfer(
            CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
            10u64.pow(base_decimals as u32) * max_contribution,
        )?;
        
        presale.owner = ctx.accounts.owner.key();
        presale.min_allocation = min_allocation;
        presale.max_allocation = max_allocation;
        presale.hardcap = hardcap;
        presale.softcap = softcap;
        presale.sale_price = sale_price;
        presale.launch_price = launch_price;
        presale.start_time = start_time;
        presale.end_time = end_time;
        presale.base_mint = base_mint;
        presale.quote_mint = quote_mint;
        presale.max_contribution = max_contribution;
        presale.total_contributions = 0;
        presale.state = 0;

        Ok(())
    }

    pub fn buy_tokens(
        ctx: Context<BuyTokens>, 
        amount: u64,
        base_decimals: u64,
        is_native: u64,
        global_bump: u8,
    ) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut presale = ctx.accounts.presale.load_mut()?;

        if timestamp > presale.end_time {
            return Err(ErrorCode::PresaleEnd.into());
        }
        if presale.total_contributions + amount > presale.hardcap {
            return Err(ErrorCode::HardcapExceeded.into());
        }
        if amount < presale.min_allocation || amount > presale.max_allocation {
            return Err(ErrorCode::InvalidContributionAmount.into());
        }
        if presale.state != 0 {
            return Err(ErrorCode::NotAvaliable.into());
        }

        let base_amount = amount / presale.sale_price;       

        /////////////////////////send quote to global from user
        let token_program = &mut &ctx.accounts.token_program;

        msg!("Native---(QUOTE-BASE).---0");
        // native token
        if is_native == 1 {
            msg!("Native---(QUOTE-BASE).---1");
            sol_transfer_user(
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.global_authority.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                amount,
            )?;
        }
        else {
            msg!("Native---(QUOTE-base)---2");
            let src_quote_account_info = &mut &ctx.accounts.user_quote_token_account;
            let dest_quote_account_info = &mut &ctx.accounts.global_quote_token_account;
            
            let cpi_accounts = Transfer {
                from: src_quote_account_info.to_account_info().clone(),
                to: dest_quote_account_info.to_account_info().clone(),
                authority: ctx.accounts.buyer.to_account_info().clone(),
            }; 
            token::transfer(
                CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
                amount,
            )?;
        }

        msg!("Native---(QUOTE-BASE).---3");
        //////////////////////////send base to user from global
        let src_base_account_info = &mut &ctx.accounts.global_base_token_account;
        let dest_base_account_info = &mut &ctx.accounts.user_base_token_account;

        let token_program = &mut &ctx.accounts.token_program;

        let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: src_base_account_info.to_account_info().clone(),
            to: dest_base_account_info.to_account_info().clone(),
            authority: ctx.accounts.global_authority.to_account_info().clone(),
        }; 
        token::transfer(
            CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
            10u64.pow(base_decimals as u32) * base_amount,
        )?;

        presale.total_contributions += amount;

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>, 
        base_decimals: u64,
        is_native: u64,
        global_bump: u8,
    ) -> Result<()> {
        let mut presale = ctx.accounts.presale.load_mut()?;

        if presale.state == 0 {
            return Err(ErrorCode::NotApproved.into());
        }
        if presale.state == 2 {
            return Err(ErrorCode::Withdrawed.into());
        }

        let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
        let signer = &[&seeds[..]];

        let base_amount = (presale.max_contribution) - presale.total_contributions / presale.sale_price;
        let quote_amount = presale.total_contributions * 75 / 100;
        let fee_amount = presale.total_contributions * 24 / 100;

        msg!("Native---(QUOTE-BASE).---base amount {}", 10u64.pow(base_decimals as u32) * base_amount);
        //////////////////////////send base to user from global
        let src_base_account_info = &mut &ctx.accounts.global_base_token_account;
        let dest_base_account_info = &mut &ctx.accounts.user_base_token_account;
        let token_program = &mut &ctx.accounts.token_program;

        let cpi_accounts = Transfer {
            from: src_base_account_info.to_account_info().clone(),
            to: dest_base_account_info.to_account_info().clone(),
            authority: ctx.accounts.global_authority.to_account_info().clone(),
        }; 

        token::transfer(
            CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
            10u64.pow(base_decimals as u32) * base_amount,
        )?;

        /////////////////////////send quote from global to client
        let token_program = &mut &ctx.accounts.token_program;

        msg!("Native---(QUOTE-BASE).---0");
        // native token
        if is_native == 1 {
            msg!("Native---(QUOTE-BASE).---1");
            sol_transfer_with_signer(
                ctx.accounts.global_authority.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                signer,
                quote_amount,
            )?;

            msg!("Native---(QUOTE-BASE).---11");
            sol_transfer_with_signer(
                ctx.accounts.global_authority.to_account_info(),
                ctx.accounts.admin.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                signer,
                fee_amount,
            )?;
        }
        else {
            msg!("Native---(QUOTE-base)---2");
            let src_quote_account_info = &mut &ctx.accounts.global_quote_token_account;
            let dest_quote_account_info = &mut &ctx.accounts.user_quote_token_account;
            
            let cpi_accounts = Transfer {
                from: src_quote_account_info.to_account_info().clone(),
                to: dest_quote_account_info.to_account_info().clone(),
                authority: ctx.accounts.global_authority.to_account_info().clone(),
            }; 
            token::transfer(
                CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
                quote_amount,
            )?;

            msg!("Native---(QUOTE-BASE).---12");
            /////////////////////////////////////////////////
            let cpi_accounts = Transfer {
                from: src_quote_account_info.to_account_info().clone(),
                to: ctx.accounts.admin_quote_token_account.to_account_info().clone(),
                authority: ctx.accounts.global_authority.to_account_info().clone(),
            }; 
            token::transfer(
                CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
                fee_amount,
            )?;
        }

        presale.total_contributions = 0;
        presale.state = 2;
        Ok(())
    }

    pub fn set_approve(
        ctx: Context<SetApprove>, 
    ) -> Result<()> {
        // let timestamp = Clock::get()?.unix_timestamp;
        let mut presale = ctx.accounts.presale.load_mut()?;

        // if timestamp < presale.end_time {
        //     return Err(ErrorCode::PresaleNotEnded.into());
        // }

        presale.state = 1;

        Ok(())
    }
}

// Accounts structure for initialize function
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
        payer = admin,
        space = 8 + 32 * 3 + 8 * 11
    )]
    pub global_authority: Account<'info, Global>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePresale<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, Global>,

    #[account(zero)]
    pub presale: AccountLoader<'info, Presale>,

    #[account(
        mut,
        constraint = creater_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = creater_token_account.owner == *owner.key,
    )]
    pub creater_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = dest_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = dest_token_account.owner == *global_authority.to_account_info().key,
    )]
    pub dest_token_account: Account<'info, TokenAccount>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_mint_address: AccountInfo<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub presale: AccountLoader<'info, Presale>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, Global>,

    #[account(mut)]
    /// CHECK:
    pub creator: AccountInfo<'info>,

    #[account(mut)]
    pub user_base_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        // constraint = global_base_token_account.owner == creator.key(),
    )]
    pub global_base_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_quote_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        // constraint = global_quote_token_account.owner == creator.key(),
    )]
    pub global_quote_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub presale: AccountLoader<'info, Presale>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, Global>,

    #[account(mut)]
    /// CHECK:
    pub creator: AccountInfo<'info>,

    #[account(mut)]
    pub user_base_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub global_base_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_quote_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub global_quote_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin.key() == ADMIN_WALLET.parse::<Pubkey>().unwrap()
    )]
    /// CHECK:
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub admin_quote_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetApprove<'info> {
    #[account(mut)]
    pub presale: AccountLoader<'info, Presale>,
}

#[account]
#[derive(Default)]
pub struct Global {
    pub admin: Pubkey, // 32
}

// Presale account structure
#[account(zero_copy)]
pub struct Presale {
    pub owner: Pubkey,
    pub min_allocation: u64,
    pub max_allocation: u64,
    pub hardcap: u64,
    pub softcap: u64,
    pub sale_price: u64, // Quote token amount per base token
    pub launch_price: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub total_contributions: u64,
    pub max_contribution: u64,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub state: u64,
}

impl Default for Presale {
    #[inline]
    #[warn(unused_must_use)]
    fn default() -> Presale {
        Presale {
            owner: Pubkey::default(),
            base_mint: Pubkey::default(),
            quote_mint: Pubkey::default(),
            min_allocation: 0,
            max_allocation: 0,
            hardcap: 0,
            softcap: 0,
            sale_price: 0,
            launch_price: 0,
            start_time: 0,
            end_time: 0,
            total_contributions: 0,
            max_contribution: 0,
            state: 0
        }
    }
}

// Error codes enumeration
#[error_code]
pub enum ErrorCode {
    #[msg("The presale is not started.")]
    PresaleStart,
    #[msg("The presale is ended.")]
    PresaleEnd,
    #[msg("The presale is not ended.")]
    PresaleNotEnded,
    #[msg("The contribution amount is invalid.")]
    InvalidContributionAmount,
    #[msg("The hardcap has been exceeded.")]
    HardcapExceeded,
    #[msg("Proposal does not approved.")]
    NotApproved,
    #[msg("Already withdrawed.")]
    Withdrawed,
    #[msg("Presale is not avaliable now.")]
    NotAvaliable,
}