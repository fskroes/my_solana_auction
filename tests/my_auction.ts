import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAuction } from "../target/types/my_auction";
import { LAMPORTS_PER_SOL, PublicKey, RpcResponseAndContext, SignatureResult, SystemProgram, TransactionSignature } from "@solana/web3.js";

describe("my_auction", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyAuction as Program<MyAuction>;

  const auction = anchor.web3.Keypair.generate();
  const treasury = anchor.web3.Keypair.generate();

  const exhibitorAccount = anchor.web3.Keypair.generate();
  const bidder1Account = anchor.web3.Keypair.generate();

  const auction_duration: anchor.BN = new anchor.BN(10);
  let initialPrice: anchor.BN = new anchor.BN(1);

  it("setup", async () => {
    const txExhibitorAirdrop = await provider.connection.requestAirdrop(exhibitorAccount.publicKey, LAMPORTS_PER_SOL * 5);
    const txBidder1Account = await provider.connection.requestAirdrop(bidder1Account.publicKey, LAMPORTS_PER_SOL * 5);
    await checkTransactionIsOK(provider, txExhibitorAirdrop);
    await checkTransactionIsOK(provider, txBidder1Account);
  });

  it("Is initialized!", async () => {
    await program.methods
      .initialize(auction_duration, initialPrice)
      .accounts({
        exhibitor: exhibitorAccount.publicKey,
        auctionAccount: auction.publicKey,
        treasury: treasury.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([exhibitorAccount, auction, treasury])
      .rpc();
    
  });
});


const checkTransactionIsOK = async function(provider: anchor.Provider, transaction: TransactionSignature): Promise<RpcResponseAndContext<SignatureResult>> { 
  const {blockhash, lastValidBlockHeight} = await provider.connection.getLatestBlockhash();
  return await provider.connection.confirmTransaction({
    blockhash,
    lastValidBlockHeight,
    signature: transaction
  });
}

const getAccountTokenBalance = async function(provider: anchor.Provider, account_pub_key: PublicKey): Promise<number> {
  return parseInt(
    (await provider.connection.getTokenAccountBalance(account_pub_key)).value.amount
  );
}