// DO NOT EDIT - automatically generated file
pub mod my_auction_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn initialize(
        client: &Client,
        i_duration: u64,
        i_amount: u64,
        a_exhibitor: anchor_lang::solana_program::pubkey::Pubkey,
        a_auction_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                my_auction::instruction::Initialize {
                    duration: i_duration,
                    amount: i_amount,
                },
                my_auction::accounts::Initialize {
                    exhibitor: a_exhibitor,
                    auction_account: a_auction_account,
                    treasury: a_treasury,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn initialize_ix(
        i_duration: u64,
        i_amount: u64,
        a_exhibitor: anchor_lang::solana_program::pubkey::Pubkey,
        a_auction_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_treasury: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: my_auction::instruction::Initialize {
                duration: i_duration,
                amount: i_amount,
            }
            .data(),
            accounts: my_auction::accounts::Initialize {
                exhibitor: a_exhibitor,
                auction_account: a_auction_account,
                treasury: a_treasury,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
