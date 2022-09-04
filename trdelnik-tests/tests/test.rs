

use fehler::throws;
use my_auction::Auction;
use program_client::my_auction_instruction::*;
use trdelnik_client::{
    anyhow::Result,
    solana_sdk::{
        native_token::{lamports_to_sol, sol_to_lamports},
        system_program, account::{Account},
    },
    *,
};

#[throws]
#[fixture]
async fn init_fixture() -> Fixture {
    let mut fixture = Fixture::new();
    fixture.deploy().await?;

    fixture
        .client
        .airdrop(fixture.exhibitor.pubkey(), 5_000_000_000)
        .await?;

    fixture
        .client
        .airdrop(fixture.bidder1.pubkey(), 5_000_000_000)
        .await?;

    initialize(
        &fixture.client,
        10,
        1,
        fixture.exhibitor.pubkey(),
        fixture.auction_account.pubkey(),
        fixture.treasury.pubkey(),
        system_program::id(),
        [
            fixture.exhibitor.clone(),
            fixture.auction_account.clone(),
            fixture.treasury.clone(),
        ],
    ).await?;

    fixture
}

#[trdelnik_test]
async fn test_exhibition_account_is_used_to_init_exhibition_sol_is_deducted(#[future] init_fixture: Result<Fixture>) {
    let mut fixture = init_fixture.await?;
    fixture.print_state().await?;

    let balance_exhibitor = lamports_to_sol(
        fixture
            .client
            .get_balance(fixture.exhibitor.pubkey())
            .await?
    );
    
    println!(
        "Exhibitor: {:?}", balance_exhibitor
    );
    assert!(balance_exhibitor < sol_to_lamports(5.0) as f64);
}

#[trdelnik_test]
async fn test_auction_account_is_initialized_owner_same_as_program_id(#[future] init_fixture: Result<Fixture>) {
    let mut fixture = init_fixture.await?;
    fixture.print_state().await?;

    let auction_account_state = fixture
        .client
        .get_account(fixture.auction_account.pubkey())
        .await?
        .unwrap();
    
    println!(
        "Treasury account {:?}", auction_account_state
    );

    assert_eq!(program_keypair(0).pubkey(), auction_account_state.owner);
}

#[trdelnik_test]
async fn test_treasury_account_is_initialized_owner_same_as_program_id(#[future] init_fixture: Result<Fixture>) {
    let mut fixture = init_fixture.await?;
    fixture.print_state().await?;

    let treasury_account_state = fixture
        .client
        .get_account(fixture.treasury.pubkey())
        .await?
        .unwrap();
    
    println!(
        "Treasury account {:?}", treasury_account_state
    );

    assert_eq!(program_keypair(0).pubkey(), treasury_account_state.owner);
}

#[trdelnik_test]
async fn test_bidding_with_higher_price_bidding_priced_raised(#[future] init_fixture: Result<Fixture>) {
    let mut fixture = init_fixture.await?;
    fixture.print_state().await?;

    bid(
        &fixture.client, 
        sol_to_lamports(2.0), 
        fixture.bid1, 
        fixture.bidder1.pubkey(), 
        fixture.auction_account.pubkey(), 
        fixture.treasury.pubkey(), 
        system_program::id(), 
        [
            fixture.bidder1.clone(),
        ]
    ).await?;

    fixture.print_state().await?;
    
    let account_state = fixture
        .client
        .get_account(fixture.auction_account.pubkey())
        .await?
        .unwrap();
    print!("{:?} \n", fixture.deserialized_auction_state(account_state));

    // let pre_bid_price = fixture.deserialized_auction_state(account_state_pre_bid)?;
    // let deserialized_auction_state = fixture.deserialized_auction_state(account_state)?;
    
    
    // assert_eq!(deserialized_auction_state.highest_bidder_pubkey, fixture.bidder1.pubkey());

}


struct Fixture {
    client: Client,
    auction_account: Keypair,
    treasury: Keypair,
    exhibitor: Keypair,
    bidder1: Keypair,
    bid1: Pubkey
}
impl Fixture {
    fn new() -> Self {

        let auction_program = program_keypair(0);
        let auction_account = keypair(42);
        let bidder1 = keypair(21);

        let (bid1, _) = Pubkey::find_program_address(
            &[auction_account.pubkey().as_ref(), bidder1.pubkey().as_ref()],
            &auction_program.pubkey(),
        );

        Fixture {
            client: Client::new(system_keypair(0)),
            auction_account: keypair(42),
            treasury: keypair(99),
            exhibitor: keypair(32),
            bidder1: keypair(21),
            bid1,
        }
    }

    #[throws]
    async fn deploy(&mut self) {
        self.client
            .airdrop(self.client.payer().pubkey(), 5_000_000_000)
            .await?;
        self.client
            .deploy_by_name(&program_keypair(0), "my_auction")
            .await?;
    }

    #[throws]
    async fn print_state(&mut self) {
        println!("\n-------------STATE---------------");
        println!(
            "initializer key: {:?} \ninitializer balance: {:?} \ntreasury key: {:?} \ntreasury balance: {:?} \nbidder1 key: {:?} \nbidder1 balance: {:?}",
            self.exhibitor.pubkey(),
            lamports_to_sol(self.client.get_balance(self.exhibitor.pubkey()).await?),
            self.treasury.pubkey(),
            lamports_to_sol(self.client.get_balance(self.treasury.pubkey()).await?),
            self.bidder1.pubkey(),
            lamports_to_sol(self.client.get_balance(self.bidder1.pubkey()).await?),
        );
        println!("---------------------------------\n");
    }

    fn deserialized_auction_state(&mut self, account_state: Account) -> Auction {
        return account_state.deserialize_data::<Auction>().unwrap();
    }
}
