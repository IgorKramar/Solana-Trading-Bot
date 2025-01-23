use super::Strategy;
use crate::jito_integration::{TradeAction, TradingSignal};
use crate::price_feed::{PriceData, TokenPair};
use std::error::Error;

pub struct MomentumStrategy {
    rsi_period: usize,
    rsi_overbought: f64,
    rsi_oversold: f64,
    volume_threshold: f64,
}

impl MomentumStrategy {
    pub fn new() -> Self {
        Self {
            rsi_period: 14,
            rsi_overbought: 70.0,
            rsi_oversold: 30.0,
            volume_threshold: 1000000.0, // в USDC
        }
    }

    fn calculate_rsi(&self, prices: &[f64]) -> f64 {
        if prices.len() < self.rsi_period + 1 {
            return 50.0;
        }

        let mut gains = 0.0;
        let mut losses = 0.0;

        for i in 1..=self.rsi_period {
            let diff = prices[i] - prices[i - 1];
            if diff > 0.0 {
                gains += diff;
            } else {
                losses -= diff;
            }
        }

        let avg_gain = gains / self.rsi_period as f64;
        let avg_loss = losses / self.rsi_period as f64;

        if avg_loss == 0.0 {
            return 100.0;
        }

        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }
}

impl Strategy for MomentumStrategy {
    fn analyze(
        &self,
        pair: &TokenPair,
        price_data: &PriceData,
    ) -> Result<Option<TradingSignal>, Box<dyn Error>> {
        let prices = price_data.get_price_history(pair, self.rsi_period + 1)?;
        let volume = price_data.get_volume(pair)?;

        // Проверяем достаточный ли объем
        if volume < self.volume_threshold {
            return Ok(None);
        }

        let rsi = self.calculate_rsi(&prices);
        let current_price = prices.last().unwrap();

        if rsi < self.rsi_oversold {
            Ok(Some(TradingSignal {
                market: get_market_address(pair)?,
                action: TradeAction::Buy,
                amount: calculate_position_size(*current_price)?,
                price: (*current_price * 1e6) as u64,
                payer: get_payer()?,
            }))
        } else if rsi > self.rsi_overbought {
            Ok(Some(TradingSignal {
                market: get_market_address(pair)?,
                action: TradeAction::Sell,
                amount: calculate_position_size(*current_price)?,
                price: (*current_price * 1e6) as u64,
                payer: get_payer()?,
            }))
        } else {
            Ok(None)
        }
    }
}
