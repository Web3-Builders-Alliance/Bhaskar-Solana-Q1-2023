/*
PROCESSING INSTRUCTIONS - ANCHOR
https://github.com/solana-developers/program-examples/blob/main/basics/processing-instructions/anchor/programs/processing-instructions/src/lib.rs
*/

use anchor_lang::prelude::*;

// creates a 'ID' field that stores address of the program
// anchor uses this 'ID' for security checks and allows other programs 
// to access your program's address
declare_id!("DgoL5J44aspizyUs9fcnpGEUJjWTLJRCfx8eYtUMYczf");

// defines the module containing all instruction handlers defining different entries into a solana program
#[program]
pub mod processing_instructions {
    use super::*;

    // With Anchor, we just put instruction data in the function signature!
    // if function requires additional instruction data they are passed after the ctx,
    // name & height - anchor will automatically deserialize this into arguments
    pub fn go_to_park(ctx: Context<Park>, name: String, height: u32) -> Result<()> {
        msg!("Welcome to the park, {}!", name);
        if height > 5 {
            msg!("You are tall enough to ride this ride. Congratulations.");
        } else {
            msg!("You are NOT tall enough to ride this ride. Sorry mate.");
        };

        Ok(())
    }
}

// here you define which accounts your instructions expects and the constraints 
// they should adhere to this struct is passed into ctx of the instruction handler
// this is where the account validation takes place
#[derive(Accounts)]
pub struct Park {}
