use anchor_lang::prelude::*;
// use solana_program::pubkey::Pubkey;
// use solana_program::{program::invoke, system_instruction};

pub mod account;
pub mod constants;
pub mod error;
pub mod utils;

use account::*;
use constants::*;
use error::*;
use utils::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod coin_flip {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _global_bump: u8, _vault_bump: u8) -> Result<()> {
        let global_authority = &mut ctx.accounts.global_authority;
        global_authority.super_admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn initialize_player_pool(ctx: Context<InitializePlayerPool>) -> Result<()> {
        let mut player_pool = ctx.accounts.player_pool.load_init()?;
        player_pool.player = ctx.accounts.owner.key();
        msg!("Owner: {:?}", player_pool.player.to_string());

        Ok(())
    }

    
    /**
    The main function to play coin_flip.
    Input Args:
    play_choice:    Tail : 0, Head : 1
    deposit:        The SOL amount to deposit
    */
    #[access_control(user(&ctx.accounts.player_pool, &ctx.accounts.owner))]
    pub fn play_game(
        ctx: Context<PlayRound>,
        _global_bump: u8,
        vault_bump: u8,
        play_choice: u64,
        deposit: u64,
    ) -> Result<()> {
        let mut player_pool = ctx.accounts.player_pool.load_mut()?;
        msg!("Deopsit: {}", deposit);
        msg!(
            "Vault: {}",
            ctx.accounts.reward_vault.to_account_info().key()
        );
        msg!(
            "Lamports: {}",
            ctx.accounts.reward_vault.to_account_info().lamports()
        );
        msg!(
            "Owner Lamports: {}",
            ctx.accounts.owner.to_account_info().lamports()
        );
        require!(
            ctx.accounts.owner.to_account_info().lamports() > deposit,
            GameError::InsufficientUserBalance
        );

        require!(
            ctx.accounts.reward_vault.to_account_info().lamports() > 2 * deposit,
            GameError::InsufficientRewardVault
        );

        // Transfer deposit Sol to this PDA
        sol_transfer_user(
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.reward_vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            deposit,
        )?;

          // Generate random number
          let mut reward: u64 = 0;
          let timestamp = Clock::get()?.unix_timestamp;
          let owner_address = ctx.accounts.owner.to_account_info().key();
          let (player_address, _bump) = Pubkey::find_program_address(
              &[
                  RANDOM_SEED.as_bytes(),
                  timestamp.to_string().as_bytes(),
                  &owner_address.to_bytes(),
              ],
              &coin_flip::ID,
          );
  
          let char_vec: Vec<char> = player_address.to_string().chars().collect();
          let number = u32::from(char_vec[0]) + u32::from(char_vec[2]) + u32::from(char_vec[4]);
          let rand = (number % 2) as u64;
          // Compare random number and play_choice
          if rand == play_choice {
              reward = 2 * deposit;
          }
  
          // Add game data to the blockchain
          player_pool.add_game_data(timestamp, deposit, reward, play_choice, rand);

          if reward > 0 {
            // Transfer SOL to the winner from the PDA
            sol_transfer_with_signer(
                ctx.accounts.reward_vault.to_account_info(),
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                &[&[VAULT_AUTHORITY_SEED.as_ref(), &[vault_bump]]],
                reward,
            )?;
        }
  
          ctx.accounts.global_authority.total_round += 1;
  
          Ok(())
      }
}


#[derive(Accounts)]
#[instruction(_global_bump: u8, _vault_bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump,
        payer = admin,
        space = 48
    )]
    pub global_authority: Account<'info, GlobalPool>,

    /// CHECK:
    #[account(
        seeds = [VAULT_AUTHORITY_SEED.as_ref()],
        bump = _vault_bump,
    )]
    pub reward_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct InitializePlayerPool<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(zero)]
    pub player_pool: AccountLoader<'info, PlayerPool>,
}

#[derive(Accounts)]
#[instruction(
    global_bump: u8,
    vault_bump: u8,
)]
pub struct PlayRound<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub player_pool: AccountLoader<'info, PlayerPool>,

    #[account(
        mut,
        seeds = [GLOBAL_AUTHORITY_SEED.as_ref()],
        bump = global_bump,
    )]
    pub global_authority: Box<Account<'info, GlobalPool>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [VAULT_AUTHORITY_SEED.as_ref()],
        bump = vault_bump,
    )]
    pub reward_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

// Access control modifiers
fn user(pool_loader: &AccountLoader<PlayerPool>, user: &AccountInfo) -> Result<()> {
    let user_pool = pool_loader.load()?;
    require!(user_pool.player == *user.key, GameError::InvalidPlayerPool);
    Ok(())
}