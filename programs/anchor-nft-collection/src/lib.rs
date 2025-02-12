use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, set_and_verify_collection,
        sign_metadata, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata,
        SetAndVerifyCollection, SignMetadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    pda::{find_master_edition_account, find_metadata_account},
    state::{Creator, DataV2},
};

declare_id!("tXpr3mdwtPuWMU29z9DGUyNM1UAJ8pHnEPbe86RR9vb");

#[constant]
pub const SEED: &str = "Collection";

#[program]
pub mod anchor_nft_collection {
    use super::*;

    pub fn create_collection_nft(
        ctx: Context<CreateCollectionNft>,
        uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        // PDA for signing
        let signer_seeds: &[&[&[u8]]] = &[&[
            SEED.as_bytes(),
            &[*ctx.bumps.get("collection_mint").unwrap()],
        ]];

        // mint collection nft
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        // create metadata account for collection nft
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(), // use pda mint address as mint authority
                    update_authority: ctx.accounts.collection_mint.to_account_info(), // use pda mint as update authority
                    payer: ctx.accounts.authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &signer_seeds,
            ),
            DataV2 {
                name: name,
                symbol: symbol,
                uri: uri,
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.authority.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        // create master edition account for collection nft
        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    payer: ctx.accounts.authority.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    edition: ctx.accounts.master_edition.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &signer_seeds,
            ),
            Some(0),
        )?;

        // verify creator on metadata account
        sign_metadata(CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            SignMetadata {
                creator: ctx.accounts.authority.to_account_info(),
                metadata: ctx.accounts.metadata_account.to_account_info(),
            },
        ))?;

        Ok(())
    }

    pub fn create_nft_in_collection(
        ctx: Context<CreateNftInCollection>,
        uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            SEED.as_bytes(),
            &[*ctx.bumps.get("collection_mint").unwrap()],
        ]];

        // mint nft in collection
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        // create metadata account for nft in collection
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.user.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &signer_seeds,
            ),
            DataV2 {
                name: name,
                symbol: symbol,
                uri: uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        // create master edition account for nft in collection
        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    payer: ctx.accounts.user.to_account_info(),
                    mint: ctx.accounts.nft_mint.to_account_info(),
                    edition: ctx.accounts.master_edition.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                &signer_seeds,
            ),
            Some(0),
        )?;

        // verify nft as part of collection
        set_and_verify_collection(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                SetAndVerifyCollection {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    collection_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.user.to_account_info(),
                    update_authority: ctx.accounts.collection_mint.to_account_info(),
                    collection_mint: ctx.accounts.collection_mint.to_account_info(),
                    collection_metadata: ctx.accounts.collection_metadata_account.to_account_info(),
                    collection_master_edition: ctx
                        .accounts
                        .collection_master_edition
                        .to_account_info(),
                },
                &signer_seeds,
            ),
            None,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCollectionNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [SEED.as_bytes()],
        bump,
        payer = authority,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub collection_mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
        mut,
        address=find_metadata_account(&collection_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        address=find_master_edition_account(&collection_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = collection_mint,
        associated_token::authority = authority
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateNftInCollection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED.as_bytes()],
        bump,
    )]
    pub collection_mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
        mut,
        address=find_metadata_account(&collection_mint.key()).0
    )]
    pub collection_metadata_account: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        address=find_master_edition_account(&collection_mint.key()).0
    )]
    pub collection_master_edition: UncheckedAccount<'info>,

    #[account(
        init,
        payer = user,
        mint::decimals = 0,
        mint::authority = collection_mint,
        mint::freeze_authority = collection_mint
    )]
    pub nft_mint: Account<'info, Mint>,

    /// CHECK:
    #[account(
        mut,
        address=find_metadata_account(&nft_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
        mut,
        address=find_master_edition_account(&nft_mint.key()).0
    )]
    pub master_edition: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}
