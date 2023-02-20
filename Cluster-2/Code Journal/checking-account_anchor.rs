/*
CHECKING ACCOUNT - ANCHOR
https://github.com/solana-developers/program-examples/blob/main/basics/checking-accounts/anchor/programs/anchor-program-example/src/lib.rs
*/

// use this import to gain access to common anchor features
use anchor_lang::prelude::*;

// creates a 'ID' field that stores address of the program
// anchor uses this 'ID' for security checks and allows other programs 
// to access your program's address
declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw");


// defines the module containing all instruction handlers defining different entries into a solana program
#[program]
pub mod anchor_program_example {
    use super::*;

    // Context is generic over account struct
    // ctx can access the ctx.accounts, ctx.program_id, ctx.remaining_accounts
    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> {
        // instruction handler is empty this program just validates 
        // the accounts passed and checks them for specified constraints
        Ok(())
    }
}
    // here you define which accounts your instructions expects and the constraints 
    // they should adhere to this struct is passed into ctx of the instruction handler
    // this is where the account validation takes place
#[derive(Accounts)]
pub struct CheckingAccounts<'info> {
    // validates that the payer signed the transaction, no other ownership checks are done
    // if this is used then you should not access underlying account data
    payer: Signer<'info>,
    #[account(mut)]  // this checks that the account_to_create is mutable or not
    /// CHECK: This account's data is empty
    account_to_create: AccountInfo<'info>, // better to use UncheckedAccount instead of AccountInfo
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_change: AccountInfo<'info>,
    // validates that account passed in is a valid program in this case the System program
    system_program: Program<'info, System>, 
}