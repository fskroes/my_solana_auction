use fehler::throws;
use program_client;
use trdelnik_client::{anyhow::Result, *};
use anchor_spl::token::Token;
use spl_associated_token_account;

use my_auction;

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();

    client: client::new(system_keypair(0)),
    
    auction_duration: 10,
    initial_price: 1,
    mint: keypair(1),
    client
        .create_token_mint(&mint, mint.pubkey(), None, 0)
        .await?;

    exhibitor: keypair(2),
    exhibitor_nft_token_account: keypair(3),
    // constructs associated token account
    let token_account = client
        .create_associated_token_account(&exhibitor, mint.pubkey())
        .await?;
    exhibitor_nft_temp_account: keypair(4),
    exhibitor_ft_receiving_account: keypair(5),
    let associated_token_program = 
        spl_associated_token_account::id();
    // derives the associated token account address for the given wallet and mint
  let associated_token_address = 
    spl_associated_token_account::get_associated_token_address(&exhibitor.pubkey(), mint);

    escrow_account: keypair(6),
    mint_authority: keypair(7),
    treasury: keypair(8),
    program: program_keypair(1),
    
    
    
    fixture.deploy().await?;
    my_auction::Initialize(
        fixture.client,
        fixture.mint.pubkey(),
        ficture.state.pubkey(),
        fixture.client.payer().pubkey(),
        System::id(),
        Some(fixture.state.clone()),

        token_program: Token::id()
    ).await?;
    fixture
}

#[trdelnik_test]
async fn test_happy_path(#[future] init_fixture: Result<Fixture>) {
    // @todo: add your happy path test scenario and the other test cases
    let default_fixture = Fixture::new();
    let fixture = init_fixture.await?;
    assert_eq!(fixture.program, default_fixture.program);
}

// @todo: design and implement all the logic you need for your fixture(s)
struct Fixture {
    client: Client,
    program: Keypair,
    state: Keypair,
}
impl Fixture {
    fn new() -> Self {
        Fixture {
            client: Client::new(system_keypair(0)),
            program: program_keypair(1),
            state: keypair(42),
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
    }
}
