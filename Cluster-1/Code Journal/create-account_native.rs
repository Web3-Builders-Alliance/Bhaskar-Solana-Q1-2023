/*
CREATE ACCOUNT - NATIVE
https://github.com/solana-developers/program-examples/blob/main/basics/create-account/native/program/src/lib.rs
*/

//importing all the necessary crates with use keyword
use solana_program::{ 
    account_info::{AccountInfo, next_account_info}, // AccountInfo - Struct which contains info on account such as pubkey, lamport, owner, data etc.
    entrypoint,  
    entrypoint::ProgramResult, // returns () on success and ProgramError on failure
    msg, // used to print message to solana log
    native_token::LAMPORTS_PER_SOL, // 1 SOL = 1_000_000_000 LAMPORTS
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    system_program,
};

// the only way to call any program, when called it's passed to the BPF Loader which processes the call
entrypoint!(process_instruction);

// function which take the address(program_id) of program to call, 
// (accounts) - list of all the accounts the program reads/writes
// (instruction_data) - addtional instructions required 
fn process_instruction(
    _program_id: &Pubkey, 
    accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
) -> ProgramResult {

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter(); 

    // The payer is the first account in array who will pay for account creation 
    let payer = next_account_info(accounts_iter)?; // 

    // The new account is the one which is being created 
    let new_account = next_account_info(accounts_iter)?;

    // system_program - native program which handles account creation, lamport transfers etc
    let system_program = next_account_info(accounts_iter)?;
    
    msg!("Program invoked. Creating a system account..."); // prints the msg to log
    msg!(" New public key will be: {}", &new_account.key.to_string());
    

    // invoke allows this program to call system_instruction to create_account
    // via cross-program-invocation(cpi)
    invoke(
        &system_instruction::create_account(
            &payer.key,                         // payer/signer is paying rent to create new_account
            &new_account.key,                   // the address of the new_account
            1 * LAMPORTS_PER_SOL,               // no of lamports sent to new_account
            0,                                  // amount of space in bytes to allocate for new_account, here it's 0
            &system_program::ID,                // owner of new_account, here assigned to system_program
        ),
        &[
            // solana runtime requires you to pass all the accounts required by the program for read/write
            // so that it can parallelize the transactions.
            payer.clone(), new_account.clone(), system_program.clone()
        ]
    )?;

    msg!("Account created succesfully.");
    Ok(()) // ProgramResult is set to ().
}

/*
1) What are the concepts (borrowing, ownership, vectors etc)

===> The main concept is utilizing the CPI functionality of solana to make programs more composable, apart from that structs, iterators, msg are used.
All the parameters to process_instructions are passed as borrowed using & i.e only the reference is being passed
.clone() is also used to create a copy of the account and then pass it

2) What is the organization?

===> The code organization is very simple, such that it only requires you to pass the required arguments 
and it will invoke the system program to create account


3)What is the contract doing? What is the mechanism? 

===> The contract is used to create a new account which is owned by the system i.e only the system can modify 
the data in new account 

4)How could it be better? More efficient? Safer?

===> This is a simple and standard program used in solana ecosystem for account creation, 
so it's pretty safe and efficient. 

5)The code could be safer and better if…..

*/