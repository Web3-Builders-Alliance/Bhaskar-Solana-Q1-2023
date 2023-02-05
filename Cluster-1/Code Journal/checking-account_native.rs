/*
CHECKING ACCOUNT - NATIVE
https://github.com/solana-developers/program-examples/blob/main/basics/checking-accounts/native/program/src/lib.rs
*/

// use shortens the path required to refer a module item
// Example: if program::ProgramError on line 13 was not declared, then each time you'll need to do 
// - solana_program::program::ProgramError instead of just ProgramError::_ where _ is any corresponding error 

use solana_program::{
    account_info::{ AccountInfo, next_account_info }, // next_account_info returns the next AccountInfo struct in the array of accounts, 
                                                    // if not found returns NotEnoughAccountKeys error.
    entrypoint,  
    entrypoint::ProgramResult,  // wrapper on rust result i.e Ok returns -> () , Err returns -> ProgramError
    msg,  // msg! macro prints message to log
    program_error::ProgramError,
    pubkey::Pubkey, 
    system_program,  // native program which is used to create new accounts, allocate account data, assign accounts to owning program, transfer lamports etc
};

// the starting point for program execution
entrypoint!(process_instruction);

// only function which is passed into the entrypoint macro where code execution starts 
fn process_instruction(
    program_id: &Pubkey,          // borrowed Pubkey of the program address 
    accounts: &[AccountInfo],     // borrowed array of AccountInfo struct
    _instruction_data: &[u8],     // optional instruction data which is a referenced byte array
) -> ProgramResult {

    /*  You can verify the program ID from the instruction is in fact 
          the program ID of your program.
    */
    // checks the program_id passed into the process_instruction is equivalent 
    // to program ID of your program(the onchain address you get when you deploy the program)
    //
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)   // returns custom incorrect program error if IDs do not match
    };
    
    /*  You can verify the list has the correct number of accounts.
        This error will get thrown by default if you 
        try to reach past the end of the iter.
    */
    // checking if minimum number of accounts are passed and in the right order 
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");  // if account array contains less than 4 accounts this msg is logged
        msg!("  payer, account_to_create, account_to_change, system_program"); // this is the actual order you needed to send the accounts in
        return Err(ProgramError::NotEnoughAccountKeys)     // error if failed to borrow a reference to account data
    };

    // Accounts passed in a vector must be in the expected order.

    // accounts passed into &[AccountInfo] should be in the expected order                       ********Am I right here?*********
    let accounts_iter = &mut accounts.iter();                     //creating a mutable iterator on the accounts array because the ownership is being assigned to differnt variables below
    let _payer = next_account_info(accounts_iter)?;               // i.e the AccountInfo of the signer which is the first account in array is assined to the payer and so on
    let account_to_create = next_account_info(accounts_iter)?;    
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.

    // Checking if account_to_create does not exist already i.e it has already been initialized
    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {       // you need to pay rent while initializing so if the 
                                                 // lamport balance of an account is 0 it means the account is not initialized yet, 
                                                 // else if lamports != 0 i.e greater than 0, it means the account is already initialized
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)

    // You can also make sure an account has been initialized.

    // Checking if account_to_change has already been initialized 
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {  // if the lamport balance of an account is 0, then the account is not initialized
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    // If we want to modify an account's data, it must be owned by our program.

    // Each account in owned by some program in solana 
    // Only the owner of the account can modify the data 
    // here we are checking if the account_to_change is actually owned by our program
    // the check is done by comparing the owner field of the AccountInfo with the program_id
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.

    // comparing the pubkey of system_program with static program ID which is a constant
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(())  // returns () on success 
}

/*
1) What are the concepts (borrowing, ownership, vectors etc)

===> iterators - next_account_info iterates through the account array and returns it
a mutable account iterator is used in this program to assign the account info to variables 

2) What is the organization?

===>The program requires you to pass the required arguments and for accounts you need to pass 
the right number of accounts in right order


3)What is the contract doing? What is the mechanism? 

===> This program is used to perform checks on insturctions passed in to ensure that the only the required
accounts in exactly specified order is passed. 

We are also checking the lamports of accounts to check if they're initialized or not

4)How could it be better? More efficient? Safer?

===> If only there was a better way to do account validation ;) 
***************Ahem Anchor to the Rescue! Maybe??**********************

5)The code could be safer and better if…..

*/