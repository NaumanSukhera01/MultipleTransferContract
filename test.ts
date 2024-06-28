

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, SystemProgram, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";
import { Sol } from "../target/types/sol";

describe("sol_transfer", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Sol as Program<Sol>;
it("init",async()=>{
  const [pda,bump]=await PublicKey.findProgramAddressSync([Buffer.from("t")],program.programId);
  console.log("pda is ",pda.toBase58())

  await program.methods.init().accounts({
    from:pda,
    payer:provider.wallet.publicKey,
    systemProgram:SystemProgram.programId
  }).signers([]).rpc();

}
)
  it("Transfers SOL to multiple recipients", async () => {
    const [pda,bump]=await PublicKey.findProgramAddressSync([Buffer.from("t")],program.programId);
    const recipients = [new PublicKey("ABWGSJNjJXCFWo1unTNAkFZ6aSdcVPeQBAFhJgpFTkf3"),new PublicKey("6aWkPFfRWyT9jY2LG87cs5vdgHcvtcVrCsE6qhGc5Etm")];

    // Airdrop SOL to the 'from' account

    const amount = new anchor.BN(10);

    try {
      await program.methods.transferSol(amount, recipients)
        .accounts({
          from: pda,
          systemProgram: SystemProgram.programId,
        })
        .remainingAccounts(recipients.map(pubkey => ({
          pubkey,
          isWritable: true,
          isSigner: false
        })))
        .signers([])
        .rpc();

      // Check balances
    
    } catch (error) {
      console.error("Error details:", error);
      throw error;
    }
  });
});
