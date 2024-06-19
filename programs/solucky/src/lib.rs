use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Token, TokenAccount, Transfer},
};
use solana_program::pubkey::Pubkey;

pub mod account;
pub mod constants;
pub mod error;
pub mod utils;

use account::*;
use constants::*;
use error::*;
use utils::*;


declare_id!("7M3YFCgtxcmTuvUWasbyWdJN4MRJ7FAWTCzVt1ua2CH4");

#[program]
pub mod solucky {
    use super::*;

    /**
     * @dev Initialize the project
     */
     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let global_authority = &mut ctx.accounts.global_authority;
        global_authority.super_admin = ctx.accounts.admin.key();
        Ok(())
    }

    /**
     * @dev Create new raffle with new arguements
     * @Context has admin, global_authority accounts.
     * and zero-account Raffle, owner's nft ATA and global_authority's nft ATA
     * and token mint address
     * @param ticket_price: ticket price
     * @param end_timestamp: the end time of raffle     
     * @param max_entrants: entrants amount to take part in this raffle
     */
     pub fn create_raffle(
        ctx: Context<CreateRaffle>,
        raffle_token_amount: u64,
        ticket_price: u64,
        end_timestamp: i64,
        max_entrants: u64,
    ) -> Result<()> {
        let mut raffle = ctx.accounts.raffle.load_init()?;
        let timestamp = Clock::get()?.unix_timestamp;
        msg!("Timestamp: {}", timestamp);

        if max_entrants > 5000 {
            return Err(RaffleError::MaxEntrantsTooLarge.into());
        }
        if timestamp > end_timestamp {
            return Err(RaffleError::EndTimeError.into());
        }

        let src_token_account_info = &mut &ctx.accounts.creater_token_account;
        let dest_token_account_info = &mut &ctx.accounts.dest_token_account;
        let token_program = &mut &ctx.accounts.token_program;

        // Case NFT Raffle
        if raffle_token_amount == 0 {
            // Transfer NFT to the PDA            
            let cpi_accounts = Transfer {
                from: src_token_account_info.to_account_info().clone(),
                to: dest_token_account_info.to_account_info().clone(),
                authority: ctx.accounts.admin.to_account_info().clone(),
            };
            token::transfer(
                CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
                1,
            )?;
        }
        else {
            let cpi_accounts = Transfer {
                from: src_token_account_info.to_account_info().clone(),
                to: dest_token_account_info.to_account_info().clone(),
                authority: ctx.accounts.admin.to_account_info().clone(),
            }; 
            token::transfer(
                CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
                raffle_token_amount,
            )?;
        }  

        raffle.creator = ctx.accounts.admin.key();
        raffle.token_mint = ctx.accounts.token_mint_address.key();
        raffle.ticket_token_program = ctx.accounts.ticket_token_program.key();
        raffle.token_amount = raffle_token_amount;
        raffle.ticket_price = ticket_price;        
        raffle.start_timestamp = timestamp;
        raffle.end_timestamp = end_timestamp;
        raffle.max_entrants = max_entrants;

        Ok(())
    }

    /**
     * @dev Update Raffle Period
     * @Context has admin and raffle accounts
     * @param end_timestamp: new end_timestamp
     */
     pub fn update_raffle_period(ctx: Context<UpdateRaffle>, end_timestamp: i64) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut raffle = ctx.accounts.raffle.load_mut()?;
        if timestamp > end_timestamp {
            return Err(RaffleError::EndTimeError.into());
        }

        if timestamp > raffle.end_timestamp {
            return Err(RaffleError::RaffleEnded.into());
        }
        // if raffle.creator != ctx.accounts.admin.key() {
        //     return Err(RaffleError::NotCreator.into());
        // }

        raffle.end_timestamp = end_timestamp;

        Ok(())
    }

    /**
     * @dev Buy tickets functions
     * @Context has buyer and raffle's account.
     * global_authority and creator address and their reap token ATAs
     * @param global_bump: global_authority's bump
     * @param amount: the amount of the tickets     
     */
    pub fn buy_tickets(
        ctx: Context<BuyTickets>, 
        amount: u64,
    ) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut raffle = ctx.accounts.raffle.load_mut()?;

        if timestamp > raffle.end_timestamp {
            return Err(RaffleError::RaffleEnded.into());
        }
        if raffle.count + amount > raffle.max_entrants {
            return Err(RaffleError::NotEnoughTicketsLeft.into());
        }

        let total_amount = amount * raffle.ticket_price;        

        // Append Tickets to the Raffle Account
        for _ in 0..amount {
            raffle.append(ctx.accounts.buyer.key());
        }

        let src_account_info = &mut &ctx.accounts.user_token_account;
        let dest_account_info = &mut &ctx.accounts.creator_token_account;
        let _admin_dest_account_info = &mut &ctx.accounts.admin_token_account;

        let token_program = &mut &ctx.accounts.token_program;

        //let token_program = &mut &raffle.ticket_token_program;

        //Case Native Sol
        if raffle.ticket_token_program == Pubkey::default() {
            sol_transfer_user(
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                total_amount,
            )?;
                
            // sol_transfer_user(
            //     ctx.accounts.buyer.to_account_info(),
            //     ctx.accounts.admin.to_account_info(),
            //     ctx.accounts.system_program.to_account_info(),
            //     total_amount * is_whitelisted / 100,
            // )?;
        } else {
            let cpi_accounts = Transfer {
                from: src_account_info.to_account_info().clone(),
                to: dest_account_info.to_account_info().clone(),
                authority: ctx.accounts.buyer.to_account_info().clone(),
            }; 
            token::transfer(
                CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
                total_amount,
            )?;
            
            // let cpi_accounts = Transfer {c
            //     from: src_account_info.to_account_info().clone(),
            //     to: admin_dest_account_info.to_account_info().clone(),
            //     authority: ctx.accounts.buyer.to_account_info().clone(),
            // };
            // token::transfer(
            //     CpiContext::new(token_program.clone().to_account_info(), cpi_accounts),
            //     total_amount_ape * is_whitelisted / 100,
            // )?;
        }               

        Ok(())
    }

    /**
     * @dev Reaveal winner function
     * @Context has buyer and raffle account address
     */
     pub fn reveal_winner(ctx: Context<RevealWinner>) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let mut raffle = ctx.accounts.raffle.load_mut()?;

        if timestamp < raffle.end_timestamp {
            return Err(RaffleError::RaffleNotEnded.into());
        }

        if raffle.whitelisted == 2 {
            return Err(RaffleError::InvalidToReveal.into());
        }

        // The Logic for Determining Winner
        let (player_address, _bump) = Pubkey::find_program_address(
            &[
                RANDOM_SEED.as_bytes(),
                timestamp.to_string().as_bytes(),
            ],
            &ctx.accounts.raffle.to_account_info().key(),
        );        

        let char_vec: Vec<char> = player_address.to_string().chars().collect();
        let mut mul = 1;
        for i in 0..7 {
            mul *= u64::from(char_vec[i as usize]);
        }
        mul += u64::from(char_vec[7]);

        let winner_index = mul % raffle.count;

        raffle.whitelisted = 2;
        raffle.winner = raffle.entrants[winner_index as usize];

        Ok(())
    }

    /**
     * @dev Claim reward function
     * @Context has claimer and global_authority account
     * raffle account and the nft ATA of claimer and global_authority.
     * @param global_bump: the global_authority's bump
     */
     pub fn claim_reward(ctx: Context<ClaimReward>, global_bump: u8) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let raffle = ctx.accounts.raffle.load_mut()?;

        if timestamp < raffle.end_timestamp {
            return Err(RaffleError::RaffleNotEnded.into());
        }

        if raffle.winner != ctx.accounts.claimer.key() {
            return Err(RaffleError::NotWinner.into());
        }

        // Transfer NFT to the winner's wallet
        let src_token_account_info = &mut &ctx.accounts.src_token_account;        
        let dest_token_account_info = &mut &ctx.accounts.claimer_token_account;
        let token_program = &mut &ctx.accounts.token_program;
        let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
        let signer = &[&seeds[..]];

        if src_token_account_info.mint != raffle.ticket_token_program {
            return Err(RaffleError::NotWinner.into());
        }

        // Case NFT Raffle
        if raffle.token_amount == 0 {
            // Transfer NFT to the PDA            
            let cpi_accounts = Transfer {
                from: src_token_account_info.to_account_info().clone(),
                to: dest_token_account_info.to_account_info().clone(),
                authority: ctx.accounts.global_authority.to_account_info(),
            };
            token::transfer(
                CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
                1,
            )?;
        }
        else {
            let cpi_accounts = Transfer {
                from: src_token_account_info.to_account_info().clone(),
                to: dest_token_account_info.to_account_info().clone(),
                authority: ctx.accounts.global_authority.to_account_info(),
            }; 
            token::transfer(
                CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
                raffle.token_amount,
            )?;
        }  
    
        Ok(())
    }
    /**
     * @dev Withdraw NFT function
     * @Context has claimer and global_authority account
     * raffle account and creator's nft ATA and global_authority's nft ATA
     * @param global_bump: global_authority's bump
     */
     pub fn withdraw_nft(ctx: Context<WithdrawNft>, global_bump: u8) -> Result<()> {
        let timestamp = Clock::get()?.unix_timestamp;
        let raffle = ctx.accounts.raffle.load_mut()?;
        let ts = timestamp;
        msg!("Timestamp: {}", ts);

        if timestamp < raffle.end_timestamp {
            return Err(RaffleError::RaffleNotEnded.into());
        }
        if raffle.creator != ctx.accounts.claimer.key() {
            return Err(RaffleError::NotCreator.into());
        }
        if raffle.count != 0 {
            return Err(RaffleError::OtherEntrants.into());
        }

        // Transfer NFT to the winner's wallet
        let src_token_account_info = &mut &ctx.accounts.src_token_account;
        let dest_token_account_info = &mut &ctx.accounts.claimer_token_account;
        let token_program = &mut &ctx.accounts.token_program;
        let seeds = &[GLOBAL_AUTHORITY_SEED.as_bytes(), &[global_bump]];
        let signer = &[&seeds[..]];

        // Case NFT Raffle
        if raffle.token_amount == 0 {
            // Transfer NFT to the PDA            
            let cpi_accounts = Transfer {
                from: src_token_account_info.to_account_info().clone(),
                to: dest_token_account_info.to_account_info().clone(),
                authority: ctx.accounts.global_authority.to_account_info(),
            };
            token::transfer(
                CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
                1,
            )?;
        }
        else {
            let cpi_accounts = Transfer {
                from: src_token_account_info.to_account_info().clone(),
                to: dest_token_account_info.to_account_info().clone(),
                authority: ctx.accounts.global_authority.to_account_info(),
            }; 
            token::transfer(
                CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, signer),
                raffle.token_amount,
            )?;
        }          

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct WithdrawNft<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(mut)]
    pub raffle: AccountLoader<'info, RafflePool>,

    #[account(
        mut,
        constraint = claimer_token_account.mint == *nft_mint_address.to_account_info().key,
        constraint = claimer_token_account.owner == *claimer.key,
    )]
    pub claimer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = src_token_account.mint == *nft_mint_address.to_account_info().key,
        constraint = src_token_account.owner == *global_authority.to_account_info().key,
    )]
    pub src_token_account: Account<'info, TokenAccount>,

    /// CHECK
    pub nft_mint_address: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(mut)]
    pub raffle: AccountLoader<'info, RafflePool>,

    #[account(
        mut,
        constraint = claimer_token_account.mint == *nft_mint_address.to_account_info().key,
        constraint = claimer_token_account.owner == *claimer.key,
    )]
    pub claimer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = src_token_account.mint == *nft_mint_address.to_account_info().key,
        constraint = src_token_account.owner == *global_authority.to_account_info().key,
    )]
    pub src_token_account: Account<'info, TokenAccount>,

    /// CHECK
    pub nft_mint_address: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct RevealWinner<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub raffle: AccountLoader<'info, RafflePool>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
        payer = admin,
        space = 40
    )]
    pub global_authority: Account<'info, GlobalPool>,

    pub system_program: Program<'info, System>,
    // pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(zero)]
    pub raffle: AccountLoader<'info, RafflePool>,

    #[account(
        mut,
        constraint = creater_token_account.mint == *token_mint_address.to_account_info().key,
        constraint = creater_token_account.owner == *admin.key,
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
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub ticket_token_program: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    
}

#[derive(Accounts)]
#[instruction(global_bump: u8)]
pub struct UpdateRaffle<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub raffle: AccountLoader<'info, RafflePool>,
}

#[derive(Accounts)]
pub struct BuyTickets<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub raffle: AccountLoader<'info, RafflePool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(mut)]
    /// CHECK
    pub creator: AccountInfo<'info>,

    #[account(
        mut,
        constraint = creator_token_account.owner == creator.key(),
    )]
    pub creator_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = admin.key() == ADMIN_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub admin: SystemAccount<'info>,

    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key(),
    )]
    pub admin_token_account: Account<'info, TokenAccount>,


    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}