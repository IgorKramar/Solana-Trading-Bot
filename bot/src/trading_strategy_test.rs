#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn setup_test_price_data() -> PriceData {
        let mut prices = HashMap::new();
        prices.insert("SOL/USDC".to_string(), 20.0);
        
        let mut history = HashMap::new();
        history.insert("SOL/USDC".to_string(), vec![19.0, 19.5, 20.0, 20.5, 21.0]);
        
        PriceData { prices, history }
    }

    #[tokio::test]
    async fn test_analyze_market() {
        let program = setup_test_program();
        let strategy = TradingStrategy::new(program);
        let price_data = setup_test_price_data();

        let signals = strategy.analyze_market(&price_data).await.unwrap();
        assert!(!signals.is_empty(), "Should generate trading signals");
    }

    #[test]
    fn test_calculate_moving_average() {
        let program = setup_test_program();
        let strategy = TradingStrategy::new(program);
        let price_data = setup_test_price_data();
        let pair = TokenPair::new("SOL", "USDC");

        let ma = strategy.calculate_moving_average(&pair, &price_data).unwrap();
        assert!(ma > 0.0, "Moving average should be positive");
    }

    #[test]
    fn test_calculate_position_size() {
        let program = setup_test_program();
        let strategy = TradingStrategy::new(program);
        
        let position_size = strategy.calculate_position_size(20.0).unwrap();
        assert!(position_size <= strategy.config.max_position_size);
    }
} 