use anchor_lang::prelude::*;
use solana_program::program::{invoke, invoke_signed};

// transfer sol
pub fn sol_transfer_with_signer<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    signers: &[&[&[u8]]; 1],
    amount: u64,
) -> Result<()> {
    // msg!("sol transfer signer.---1");
    // msg!("Source: {}, Destination: {}, Amount: {}", source.key, destination.key, amount);
    // Check the lamports in the source account
    // msg!("Source account lamports: {}", source.lamports());
    // Check the source account data length and content (if any)
    // msg!("Source account data length: {}", source.data_len());
    // msg!("Source account data: {:?}", &source.data.borrow()[..]);
    
    // Additional check for source account program id
    // msg!("Source account owner: {}", source.owner);
    // let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    // msg!("sol transfer signer.---2");
    // msg!("Instruction: {:?}", ix);
    // invoke_signed(&ix, &[source.clone(), destination.clone(), system_program.clone()], signers)?;
    **source.try_borrow_mut_lamports()? -= amount;
    **destination.try_borrow_mut_lamports()? += amount;
    // msg!("sol transfer signer.---3");
    Ok(())
}

pub fn sol_transfer_user<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    Ok(invoke(&ix, &[source, destination, system_program])?)
}