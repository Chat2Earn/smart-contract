import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { C2eSmartContract } from "../target/types/c2e_smart_contract";

describe("c2e-smart-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.C2eSmartContract as Program<C2eSmartContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
