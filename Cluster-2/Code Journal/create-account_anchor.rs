/*
CREATE ACCOUNT - ANCHOR
https://github.com/solana-developers/program-examples/blob/main/basics/create-account/anchor/programs/create-system-account/src/lib.rs
*/

// prelude contains all the commonly used components of the crate
use anchor_lang::prelude::*;
use anchor_lang::system_program;

// creates a 'ID' field that stores address of the program
// anchor uses this 'ID' for security checks and allows other programs 
// to access your program's address
declare_id!("6gUwvaZPvC8ZxKuC1h5aKz4mRd7pFyEfUZckiEsBZSbk");

// There are 1-billion lamports in one SOL
const LAMPORTS_PER_SOL: u64 = 1000000000;

// program module is where you write business logic and contains instruction handlers
#[program]
pub mod create_system_account {
    use super::*;

    pub fn create_system_account(ctx: Context<CreateSystemAccount>) -> Result<()> {
        // msg macro prints the message to the console
        msg!("Program invoked. Creating a system account...");
        msg!("  New public key will be: {}", &ctx.accounts.new_account.key().to_string());

        // here Cross-Program-Invocation is used to call the system_programa to create an account
        // CPI is used by programs to interact with each other, makes the code reusable
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.payer.to_account_info(),         // From pubkey
                    to: ctx.accounts.new_account.to_account_info(),     // To pubkey
                },
            ),
            1 * LAMPORTS_PER_SOL,                           // Lamports (1 SOL)
            0,                                         // Space
            &ctx.accounts.system_program.key(),         // Owner
        )?;

        msg!("Account created succesfully.");
        Ok(())
    }
}

// here you define which accounts your instructions expects and the constraints 
// they should adhere to this struct is passed into ctx of the instruction handler
// this is where the account validation takes place
#[derive(Accounts)]
pub struct CreateSystemAccount<'info> {
    // validats that payer signed the transaction and is mutable because payer is 
    // paying for the transaction so the lamports are deducted 
    #[account(mut)] 
    pub payer: Signer<'info>,
    // checks if new_account signed the transaction
    #[account(mut)]
    pub new_account: Signer<'info>,
    // checks whether the correct System Program is passed in or not 
    pub system_program: Program<'info, System>,
}