use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use serum_dex::matching::{OrderType, Side};

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod trading_program {
    use super::*;

    pub fn initialize_trading_account(ctx: Context<InitializeTradingAccount>) -> Result<()> {
        let trading_account = &mut ctx.accounts.trading_account;
        trading_account.owner = ctx.accounts.owner.key();
        trading_account.is_active = true;
        Ok(())
    }

    pub fn create_trade_order(
        ctx: Context<CreateTradeOrder>,
        amount: u64,
        price: u64,
        side: Side,
        order_type: OrderType,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        order.owner = ctx.accounts.owner.key();
        order.market = ctx.accounts.market.key();
        order.amount = amount;
        order.price = price;
        order.side = side;
        order.order_type = order_type;
        order.status = OrderStatus::Pending;

        // Проверяем достаточность средств
        if side == Side::Bid {
            require!(
                ctx.accounts.user_token_account.amount >= amount * price,
                TradingError::InsufficientFunds
            );
        } else {
            require!(
                ctx.accounts.user_token_account.amount >= amount,
                TradingError::InsufficientFunds
            );
        }

        Ok(())
    }

    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        let order = &mut ctx.accounts.order;
        require!(
            order.owner == ctx.accounts.owner.key(),
            TradingError::Unauthorized
        );
        order.status = OrderStatus::Cancelled;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeTradingAccount<'info> {
    #[account(init, payer = owner, space = 8 + 32 + 1)]
    pub trading_account: Account<'info, TradingAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TradingAccount {
    pub owner: Pubkey,
    pub is_active: bool,
}

#[derive(Accounts)]
pub struct CreateTradeOrder<'info> {
    #[account(init, payer = owner, space = 8 + ORDER_SIZE)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub market: AccountInfo<'info>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub order: Account<'info, Order>,
    pub owner: Signer<'info>,
}

#[account]
pub struct Order {
    pub owner: Pubkey,
    pub market: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub status: OrderStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum OrderStatus {
    Pending,
    Filled,
    Cancelled,
}

const ORDER_SIZE: usize = 32 + 32 + 8 + 8 + 1 + 1 + 1;

#[error_code]
pub enum TradingError {
    #[msg("Недостаточно средств для создания ордера")]
    InsufficientFunds,
    #[msg("Неавторизованный доступ")]
    Unauthorized,
}
