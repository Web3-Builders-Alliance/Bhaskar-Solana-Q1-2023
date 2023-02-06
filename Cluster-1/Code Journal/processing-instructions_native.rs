/*
PROCESSING INSTRUCTIONS - NATIVE
https://github.com/solana-developers/program-examples/blob/main/basics/processing-instructions/native/program/src/lib.rs
*/


// Serialization refers to how data structures are translated from the client side
// into raw bytes so that data can be passed into smart contract 
// as a parameter into functions or to store it as state data

// BorshSerialize = Data Structure --> Binary format
// BorshDeserialize = Binary format --> Data Structure
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,                                        
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey,
};

// Program entry point supported by the latest BPFLoader.
entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],       
    instruction_data: &[u8],
) -> ProgramResult {

    // Attempt to serialize the BPF format to our struct
    //  using Borsh
    //
    
    // &instruction_data is passed into the program in the form &[u8] i.e slice of byte array

    // try_from_slice method helps you to deserialize this byte array into the 
    // data format specified by InstructionData struct and stores in 
    // instruction_data_object variable so that we can perform operations on this variable

    let instruction_data_object = InstructionData::try_from_slice(&instruction_data)?;

    msg!("Welcome to the park, {}!", instruction_data_object.name);  // we can access the fields of struct(InstructionData) after deserializing by using .(dot) notation
    if instruction_data_object.height > 5 {                          // we can also perform comparision operations on fields of deserialized struct
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };

    Ok(())
}

// InstructionData is the data format using which we can serialize and deserialize 
// the instruction data, we can think of this like a blueprint

// derive attribute - implements the required traits for the struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]  
pub struct InstructionData {
    name: String,
    height: u32,
}


/*
1) What are the concepts (borrowing, ownership, vectors etc)

===> The main concept is the process of serialization and deserialization of data 

On-chain all the data is stored in the form of byte representation, but for us humans it's tough 
to understand the binary format, so we represent data in normal high level language format using english
like syntax using varioud data types, data structures, classes, objects, functions etc

Serialization is the process of converting this human readable format into binary format so that we can
send it to blockchain for operations or storage. 

Deserialization if the opposite process of serialization, it takes the data which is returned in binary 
format and converts into high-level so that it becomes easy for us to read and understand
 
2) What is the organization?

===> The code organization is simple such that, you have to pass the program id, account that the program
will interact with and the most important thing is you need to pass the struct in serialized format
from client side into instruction_data.

3)What is the contract doing? What is the mechanism? 

===> The program is basically taking the serialized input passed into the process_instruction function
and using try_from_slice method deserializing it into struct object and performing comparisions on its
fields and logging it

4)How could it be better? More efficient? Safer?
 
5)The code could be safer and better if…..

*/