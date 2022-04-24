use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

declare_id!("AXuUjUsKiZ1goUtaVZYtVBdRk7WGmG4UjrLusjyb8nLF");

//PROCESSOR - the business logic in the program
#[program]
pub mod escrow {

    use anchor_lang::solana_program::entrypoint::ProgramResult;
    use anchor_spl::token;
    use spl_token::instruction::AuthorityType;

    use super::*;

    const ESCROW_PDA_SEED: &[u8] = b"escrow";

    pub fn initialize(
        ctx: Context<Initialize>,
        vault_account: u8,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts.escrow_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts
            .escrow_account
            .initializer_deposit_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;

        ctx.accounts
            .escrow_account
            .initializer_receive_token_account = *ctx
            .accounts
            .initializer_deposit_token_account
            .to_account_info()
            .key;

        ctx.accounts.escrow_account.initializer_amount = initializer_amount;
        ctx.accounts.escrow_account.taker_amount = taker_amount;

        let (vault_authority, vault_account) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);

        token::set_authority(
            ctx.accounts.into_set_authority_contex(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.escrow_account.initializer_amount,
        )?;

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> ProgramResult {
        Ok(())
    }
}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}

//INSTRUCTIONS TO THE PROGRAM
#[derive(Accounts)]
#[instruction(vault_account: u8, initializer_amount: u64)]
pub struct Initialize<'info> {
    //Signer of InitialEscrow( escrow means a bond) to be stored in the EscrowAccount
    #[account(mut, signer)]
    pub initializer: AccountInfo<'info>,
    //the account of token for token exchange.
    #[account(mut, constraint = initializer_deposit_token_account.amount >= initializer_amount)]
    pub initializer_deposit_token_account: AccountInfo<'info>,
    //the account of token for token exchange.
    pub initializer_receive_token_account: AccountInfo<'info>,
    //the account of TokenProgram
    pub token_program: AccountInfo<'info>,
    //the account of EscrowAccount
    #[account(zero)]
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    //the account of vault, which is created by Anchor via constraints.
    #[account(init, seeds=[b"token_seed".as_ref()], bump, payer=initializer, token::mint = mint, token::authority = initializer)]
    pub vault_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    pub initializer: AccountInfo<'info>,
    pub initializer_deposit_token_account: AccountInfo<'info>,
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    pub taker: AccountInfo<'info>,
    pub taker_deposit_token_account: Account<'info, TokenAccount>,
    pub taker_receive_token_account: Account<'info, TokenAccount>,
    pub initializer_deposit_token_account: Account<'info, TokenAccount>,
    pub initializer_receive_token_account: Account<'info, TokenAccount>,
    pub initializer: AccountInfo<'info>,
    pub escrow_account: Box<Account<'info, EscrowAccount>>,
    pub vault_account: Account<'info, TokenAccount>,
    pub vault_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}
