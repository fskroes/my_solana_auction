import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyAuction } from "../target/types/my_auction";
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, RpcResponseAndContext, SignatureResult, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction, TransactionInstruction, TransactionSignature } from "@solana/web3.js";
import { AccountLayout, AuthorityType, createAccount, createInitializeAccountInstruction, createMint, mintTo, setAuthority, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("my_auction", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MyAuction as Program<MyAuction>;

  let pda: PublicKey = null;

  const auction = anchor.web3.Keypair.generate();
  const treasury = anchor.web3.Keypair.generate();
  const exhibitor = anchor.web3.Keypair.generate();
  const mint = anchor.web3.Keypair.generate();

  const exhibitorAccount = anchor.web3.Keypair.generate();
  const exhibitorNftTempAccount = anchor.web3.Keypair.generate();
  
  let nftMintPubkey: PublicKey
  let ftMintPubkey: PublicKey
  let exhibitorNftTokenAccountPubkey: PublicKey
  let exhibitorFtTokenAccountPubkey: PublicKey

  it("setup", async () => {
    const txExhibitorAirdrop = await provider.connection.requestAirdrop(exhibitorAccount.publicKey, LAMPORTS_PER_SOL * 2);
    await checkTransactionIsOK(provider, txExhibitorAirdrop);


    nftMintPubkey = await createMint(provider.connection, exhibitorAccount, exhibitorAccount.publicKey, null, 0, undefined, undefined, TOKEN_PROGRAM_ID);
    console.log(`Created NFT ${nftMintPubkey}`)

    exhibitorNftTokenAccountPubkey = await createAccount(provider.connection, exhibitorAccount, nftMintPubkey, exhibitorAccount.publicKey, undefined, undefined, TOKEN_PROGRAM_ID);
    await mintTo(provider.connection, exhibitorAccount, nftMintPubkey, exhibitorNftTokenAccountPubkey, exhibitorAccount, 1, [], undefined, TOKEN_PROGRAM_ID);
    await setAuthority(provider.connection, exhibitorAccount, nftMintPubkey, exhibitorAccount, AuthorityType.MintTokens, null);

    ftMintPubkey = await createMint(provider.connection, exhibitorAccount, exhibitorAccount.publicKey, null, 0, undefined, undefined, TOKEN_PROGRAM_ID);
    console.log(`Created FT ${ftMintPubkey}`)

    exhibitorFtTokenAccountPubkey = await createAccount(provider.connection, exhibitorAccount, ftMintPubkey, exhibitorAccount.publicKey, undefined, undefined, TOKEN_PROGRAM_ID);
    await mintTo(provider.connection, exhibitorAccount, ftMintPubkey, exhibitorFtTokenAccountPubkey, exhibitorAccount, 0, [], undefined, TOKEN_PROGRAM_ID);

    
    await getCurrentStateOfAuction();
  });


  const auction_duration: anchor.BN = new anchor.BN(10);
  let initialPrice: anchor.BN = new anchor.BN(1);

  it("Is initialized!", async () => {
    const txPayerAirdrop = await provider.connection.requestAirdrop(exhibitor.publicKey, LAMPORTS_PER_SOL * 2);
    checkTransactionIsOK(provider, txPayerAirdrop);

    const [_pda, _nonce] = await PublicKey.findProgramAddress(
      [mint.publicKey.toBytes()],
      program.programId
    );

    pda = _pda;

    await program.methods
      .initialize(auction_duration, initialPrice)
      .preInstructions([
        ...await getAccountInstructions(provider.connection, nftMintPubkey, exhibitorNftTempAccount.publicKey, exhibitorAccount.publicKey),
      ])
      .accounts({
        mint: mint.publicKey,
        exhibitor: exhibitorAccount.publicKey,
        exhibitorNftTokenAccount: exhibitorNftTokenAccountPubkey,
        exhibitorNftTempAccount: exhibitorNftTempAccount.publicKey,
        exhibitorFtReceivingAccount: exhibitorFtTokenAccountPubkey,
        escrowAccount: auction.publicKey,
        mintAuthority: pda,
        treasury: treasury.publicKey,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([exhibitorAccount, mint, auction, exhibitorNftTempAccount])
      .rpc();
    
  });

  const getCurrentStateOfAuction = async function(): Promise<void> {
    const data = {
      escrow_account: {
        "Wallet Pubkey": auction.publicKey.toBase58(),
      },
      exhibitor: {
        "Wallet Pubkey": exhibitorAccount.publicKey.toBase58(),
        FT: await getAccountTokenBalance(provider, exhibitorFtTokenAccountPubkey),
        "FT(NAO) Account PubKey": exhibitorFtTokenAccountPubkey.toBase58(),
        NFT: await getAccountTokenBalance(provider, exhibitorNftTokenAccountPubkey),
        "NFT(X) Account PubKey": exhibitorNftTokenAccountPubkey.toBase58(),
      },
      // bidder1: {
      //   "Wallet Pubkey": bidder1Account.publicKey.toBase58(),
      //   FT: await getAccountTokenBalance(provider, bidder1FtTokenAccountPubkey),
      //   "FT(NAO) Account PubKey": bidder1FtTokenAccountPubkey.toBase58(),
      //   NFT: 0,
      //   "NFT(X) Account PubKey": "",
      // },
      // bidder2: {
      //   "Wallet Pubkey": bidder2Account.publicKey.toBase58(),
      //   FT: await getAccountTokenBalance(provider, bidder2FtTokenAccountPubkey),
      //   "FT(NAO) Account PubKey": bidder2FtTokenAccountPubkey.toBase58(),
      //   NFT: 0,
      //   "NFT(X) Account PubKey": "",
      // },
    };
    
    console.table(data);
  }
});


const checkTransactionIsOK = async function(provider: anchor.Provider, transaction: TransactionSignature): Promise<RpcResponseAndContext<SignatureResult>> { 
  const {blockhash, lastValidBlockHeight} = await provider.connection.getLatestBlockhash();
  return await provider.connection.confirmTransaction({
    blockhash,
    lastValidBlockHeight,
    signature: transaction
  });
}
const getAccountInstructions = async function(connection: Connection, mintPubkey: PublicKey, taPubkey: PublicKey, creatorPubkey: PublicKey): Promise<[TransactionInstruction, TransactionInstruction]> {
  const createAccount = SystemProgram.createAccount({
    space: AccountLayout.span,
    lamports: await connection.getMinimumBalanceForRentExemption(
        AccountLayout.span
    ),
    fromPubkey: creatorPubkey,
    newAccountPubkey: taPubkey,
    programId: TOKEN_PROGRAM_ID,
  });
  const initAccount = createInitializeAccountInstruction(
      taPubkey,
      mintPubkey,
      creatorPubkey,
      TOKEN_PROGRAM_ID
  );
  return [createAccount, initAccount]
}
const getAccountTokenBalance = async function(provider: anchor.Provider, account_pub_key: PublicKey): Promise<number> {
  return parseInt(
    (await provider.connection.getTokenAccountBalance(account_pub_key)).value.amount
  );
}