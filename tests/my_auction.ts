import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAuction } from "../target/types/my_auction";

describe("my_auction", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyAuction as Program<MyAuction>;



  it("Is initialized!", async () => {
    //
  });
});
