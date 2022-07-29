import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAuction } from "../target/types/my_auction";

describe("my_auction", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyAuction as Program<MyAuction>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
