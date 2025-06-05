use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("91umWRizDdvdDBzE92TCAZzrywDzp7NzyAD92J5eV8Rv");

#[program]
pub mod c2e_smart_contract {
    use super::*;

    pub fn register_username(ctx: Context<RegisterUsername>, username: String) -> Result<()> {
        // Logic to mint an NFT with metadata containing the username
        // ...
        Ok(())
    }

    pub fn reward_user(ctx: Context<RewardUser>) -> Result<()> {
        let stats = &mut ctx.accounts.user_stats;
        stats.message_count += 1;
        stats.reward_balance += 10; // Example: +10 reward units per message
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let stats = &mut ctx.accounts.user_stats;
        let amount = stats.reward_balance;
        require!(amount > 0, ErrorCode::NoRewardAvailable);

        let bump = *ctx.bumps.get("reward_vault_authority").unwrap();
        let signer_seeds = &[b"reward_vault", &[bump]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.reward_vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.reward_vault_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &[signer_seeds],
        );
        token::transfer(cpi_ctx, amount)?;

        stats.reward_balance = 0;
        Ok(())
    }

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64) -> Result<()> {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &ctx.accounts.receiver.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
            ],
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterUsername<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // Accounts for minting username NFT will go here
    // ...
}

#[derive(Accounts)]
pub struct RewardUser<'info> {
    #[account(mut)]
    pub user_stats: Account<'info, UserStats>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(
        mut,
        seeds = [b"user_stats", user.key().as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserStats>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    /// CHECK: reward vault authority PDA
    #[account(seeds = [b"reward_vault"], bump)]
    pub reward_vault_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This is safe because we trust the receiver's address
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserStats {
    pub user: Pubkey,
    pub message_count: u64,
    pub reward_balance: u64,
    pub bump: u8,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No rewards available to claim.")]
    NoRewardAvailable,
}
