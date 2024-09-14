import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CoinFlip } from "../target/types/coin_flip";

describe("coin_flip", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CoinFlip as Program<CoinFlip>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
