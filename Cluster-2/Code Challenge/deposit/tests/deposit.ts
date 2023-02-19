import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Deposit } from "../target/types/deposit";
import { Keypair, PublicKey, Connection, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";
import { BN } from "bn.js";

describe("Deposit program test suite", async () => {
      // Configure the client to use the local cluster.
      anchor.setProvider(anchor.AnchorProvider.env());

      const program = anchor.workspace.Deposit as Program<Deposit>;
      const connection: Connection = program.provider.connection;
      const user = Keypair.generate();
        
      // Fetch Vault PDA
      const [vaultPda, vaultBump] = await PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), user.publicKey.toBuffer()],
        program.programId
      );
before(async()=>{

      // Request an airdrop of 2 Sol to the user's account.
      let airdropBlock = await connection.getLatestBlockhash('finalized');
      try {
        const airdrop = await connection.requestAirdrop(user.publicKey, 2 * LAMPORTS_PER_SOL);
        await connection.confirmTransaction({
          signature: airdrop,
          blockhash: airdropBlock.blockhash,
          lastValidBlockHeight: airdropBlock.lastValidBlockHeight
        });
      } catch (error) {
        console.error("Error while requesting airdrop:", error);
      }

})
  it("Creates a vault account", async () => {

    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    try {
      const tx = await program.methods.initialize()
        .accounts({
          initializer: user.publicKey,
          vault: vaultPda,
          systemProgram: SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
      });
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`); 
    } catch (error) {
      console.error("Error while creating a vault account:", error);
    }

    // Fetch the vault account and make assertions.
    let vault;
    try {
      vault = await program.account.vault.fetch(vaultPda);
    } catch (error) {
      console.error("Error while fetching the vault account:", error);
    }
   console.log(`-----------VAULT CONTENT---------`);
    console.log('owner:   ',vault.owner.toBase58());
    console.log('bump:    ',vault.bump);
    console.log('balance: ',vault.balance.toNumber());

    assert(vault.owner.toBase58() === user.publicKey.toBase58(), "The vault owner should be the user.");
    assert(vaultBump == vault.bump, "Vault bump as expected.");
    assert(vault.balance.toNumber()== 0, "Vault balance is 0.");
  });
  it("Deposits 1 SOL", async () => {

    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    let depositAmt = new BN(LAMPORTS_PER_SOL);
    try {
      const tx = await program.methods.deposit(depositAmt)
        .accounts({
          owner: user.publicKey,
          vault: vaultPda,
          systemProgram: SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },'finalized');
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`); 
    } catch (error) {
      console.error("Error making deposit:", error);
    }

    // Fetch the vault account and make assertions.
    let vault;
    try {
      vault = await program.account.vault.fetch(vaultPda);
    } catch (error) {
      console.error("Error while fetching the vault account:", error);
    }
   console.log(`-----------VAULT CONTENT---------`);
    console.log('owner:   ',vault.owner.toBase58());
    console.log('bump:    ',vault.bump);
    console.log('balance: ',vault.balance.toNumber());

    assert(vault.balance.toNumber()== LAMPORTS_PER_SOL, "Vault balance is 1 SOL.");
  });
  it("Withdraws 0.5 SOL", async () => {

    let latestBlockhash = await connection.getLatestBlockhash('finalized');
    let withdrawAmt = new BN(0.5*LAMPORTS_PER_SOL);
    try {
      const tx = await program.methods.withdraw(withdrawAmt)
        .accounts({
          owner: user.publicKey,
          vault: vaultPda,
          systemProgram: SystemProgram.programId
        })
        .signers([user])
        .rpc();
      await connection.confirmTransaction({
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
      });
      console.log(`--------------TX URL------------`);
      console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`); 
    } catch (error) {
      console.error("Error making withdraw:");
    }

    // Fetch the vault account and make assertions.
    let vault;
    try {
      vault = await program.account.vault.fetch(vaultPda);
    } catch (error) {
      console.error("Error while fetching the vault account:", error);
    }
    console.log(`-----------VAULT CONTENT---------`);
    console.log('owner:   ',vault.owner.toBase58());
    console.log('bump:    ',vault.bump);
    console.log('balance: ',vault.balance.toNumber());

    assert(vault.balance.toNumber()== 0.5*LAMPORTS_PER_SOL, "Vault balance is 0.5 SOL.");

  });
});