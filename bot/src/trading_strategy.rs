use anchor_client::Program;
use solana_sdk::pubkey::Pubkey;
use std::error::Error;
use crate::price_feed::{PriceData, TokenPair};
use crate::jito_integration::{TradingSignal, TradeAction};

pub struct TradingStrategy {
    program: Program,
    config: StrategyConfig,
}

pub struct StrategyConfig {
    min_price_change: f64,
    max_position_size: u64,
    risk_percentage: f64,
    trading_pairs: Vec<TokenPair>,
}

impl TradingStrategy {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            config: StrategyConfig {
                min_price_change: 0.02, // 2% минимальное изменение цены
                max_position_size: 1000_000_000, // 1 SOL в ламопртах
                risk_percentage: 0.01, // 1% риска на сделку
                trading_pairs: vec![
                    TokenPair::new("SOL", "USDC"),
                    TokenPair::new("RAY", "USDC"),
                ],
            },
        }
    }

    pub async fn analyze_market(&self, price_data: &PriceData) -> Result<Vec<TradingSignal>, Box<dyn Error>> {
        let mut signals = Vec::new();

        for pair in &self.config.trading_pairs {
            if let Some(signal) = self.analyze_pair(pair, price_data).await? {
                signals.push(signal);
            }
        }

        Ok(signals)
    }

    async fn analyze_pair(&self, pair: &TokenPair, price_data: &PriceData) -> Result<Option<TradingSignal>, Box<dyn Error>> {
        let current_price = price_data.get_price(pair)?;
        let moving_average = self.calculate_moving_average(pair, price_data)?;
        
        // Простая стратегия пересечения скользящей средней
        if current_price > moving_average * (1.0 + self.config.min_price_change) {
            // Сигнал на покупку
            let amount = self.calculate_position_size(current_price)?;
            return Ok(Some(TradingSignal {
                market: self.get_market_address(pair)?,
                action: TradeAction::Buy,
                amount,
                price: (current_price * 1e6) as u64, // Конвертируем в микро-USDC
                payer: self.program.payer(),
            }));
        } else if current_price < moving_average * (1.0 - self.config.min_price_change) {
            // Сигнал на продажу
            let amount = self.calculate_position_size(current_price)?;
            return Ok(Some(TradingSignal {
                market: self.get_market_address(pair)?,
                action: TradeAction::Sell,
                amount,
                price: (current_price * 1e6) as u64,
                payer: self.program.payer(),
            }));
        }

        Ok(None)
    }

    fn calculate_moving_average(&self, pair: &TokenPair, price_data: &PriceData) -> Result<f64, Box<dyn Error>> {
        // Реализация расчета скользящей средней
        Ok(price_data.get_average_price(pair, 20)?) // 20-периодная MA
    }

    fn calculate_position_size(&self, current_price: f64) -> Result<u64, Box<dyn Error>> {
        let account_balance = self.get_account_balance().await?;
        let position_size = (account_balance as f64 * self.config.risk_percentage) / current_price;
        Ok((position_size.min(self.config.max_position_size as f64)) as u64)
    }

    async fn get_account_balance(&self) -> Result<u64, Box<dyn Error>> {
        // Получение баланса аккаунта
        Ok(self.program.rpc().get_balance(&self.program.payer()).await?)
    }

    fn get_market_address(&self, pair: &TokenPair) -> Result<Pubkey, Box<dyn Error>> {
        // Получение адреса рынка для торговой пары
        // В реальном приложении здесь будет логика получения адреса из Serum DEX
        Ok(Pubkey::new_unique())
    }
} 