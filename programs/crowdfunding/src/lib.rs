use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod crowdfunding {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        name: String,
        description: String,
    ) -> entrypoint::ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        campaign.name = name;
        campaign.description = description;
        campaign.amount_donated = 0;
        campaign.admin = *ctx.accounts.user.key;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> entrypoint::ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        if campaign.admin != *user.key {
            return Err(ProgramError::IllegalOwner);
        }

        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        if **campaign.to_account_info().lamports.borrow() < amount + rent_balance {
            return Err(ProgramError::InsufficientFunds);
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space=9000, seeds=[b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Campaign {
    pub name: String,
    pub description: String,
    pub amount_donated: u64,
    pub admin: Pubkey,
}
