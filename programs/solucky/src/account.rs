use anchor_lang::prelude::*;
use std::clone::Clone;

use crate::constants::*;

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub super_admin: Pubkey, // 32
}

#[account(zero_copy)]
pub struct RafflePool {
    // 8 + 32 * 3 + 8 * 8 + 32 + 32 * 1024 + 32 = 33000
    pub creator: Pubkey,                    //32
    pub token_program: Pubkey,              //32
    //Case NFT
    pub token_mint: Pubkey,                   //32
    //Case Raffle Token
    pub token_amount: u64,                  //8
    pub ticket_token_program: Pubkey,       //32
    pub ticket_price: u64,                  //8
    pub count: u64,                         //8
    pub no_repeat: u64,                     //8
    pub max_entrants: u64,                  //8
    pub start_timestamp: i64,               //8
    pub end_timestamp: i64,                 //8
    pub whitelisted: u64,                   //8    
    pub winner: Pubkey,                     //32
    pub entrants: [Pubkey; MAX_ENTRANTS],   //32*1024
}

impl Default for RafflePool {
    #[inline]
    #[warn(unused_must_use)]
    fn default() -> RafflePool {
        RafflePool {
            creator: Pubkey::default(),
            token_program: Pubkey::default(),
            token_mint: Pubkey::default(),
            token_amount: 0,
            ticket_token_program: Pubkey::default(),
            ticket_price: 0,
            count: 0,
            no_repeat: 0,
            max_entrants: 0,
            start_timestamp: 0,
            end_timestamp: 0,
            whitelisted: 0,
            winner: Pubkey::default(),
            entrants: [Pubkey::default(); MAX_ENTRANTS],
        }
    }
}
impl RafflePool {
    pub fn append(&mut self, buyer: Pubkey) {
        self.entrants[self.count as usize] = buyer;
        self.count += 1;
    }
}