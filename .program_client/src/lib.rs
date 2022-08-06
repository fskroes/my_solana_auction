// DO NOT EDIT - automatically generated file
pub mod my_auction_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        157u8, 126u8, 126u8, 57u8, 126u8, 97u8, 34u8, 171u8, 24u8, 22u8, 188u8, 10u8, 163u8, 102u8,
        250u8, 173u8, 53u8, 243u8, 73u8, 211u8, 147u8, 131u8, 170u8, 115u8, 136u8, 161u8, 140u8,
        153u8, 254u8, 119u8, 166u8, 226u8,
    ]);
    pub async fn initialize(
        client: &Client,
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor_nft_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor_nft_temp_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor_ft_receiving_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_rent: anchor_lang::solana_program::pubkey::Pubkey,
        a_clock: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                my_auction::instruction::Initialize {},
                my_auction::accounts::Initialize {
                    mint: a_mint,
                    exhibitor: a_exhibitor,
                    exhibitor_nft_token_account: a_exhibitor_nft_token_account,
                    exhibitor_nft_temp_account: a_exhibitor_nft_temp_account,
                    exhibitor_ft_receiving_account: a_exhibitor_ft_receiving_account,
                    escrow_account: a_escrow_account,
                    mint_authority: a_mint_authority,
                    rent: a_rent,
                    clock: a_clock,
                    system_program: a_system_program,
                    token_program: a_token_program,
                },
                signers,
            )
            .await?)
    }
    pub fn initialize_ix(
        a_mint: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor_nft_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor_nft_temp_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_exhibitor_ft_receiving_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_mint_authority: anchor_lang::solana_program::pubkey::Pubkey,
        a_rent: anchor_lang::solana_program::pubkey::Pubkey,
        a_clock: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: my_auction::instruction::Initialize {}.data(),
            accounts: my_auction::accounts::Initialize {
                mint: a_mint,
                exhibitor: a_exhibitor,
                exhibitor_nft_token_account: a_exhibitor_nft_token_account,
                exhibitor_nft_temp_account: a_exhibitor_nft_temp_account,
                exhibitor_ft_receiving_account: a_exhibitor_ft_receiving_account,
                escrow_account: a_escrow_account,
                mint_authority: a_mint_authority,
                rent: a_rent,
                clock: a_clock,
                system_program: a_system_program,
                token_program: a_token_program,
            }
            .to_account_metas(None),
        }
    }
}
