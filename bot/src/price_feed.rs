use std::collections::HashMap;
use std::error::Error;
use pyth_sdk_solana::PriceAccount;
use solana_client::rpc_client::RpcClient;

pub struct PriceFeed {
    rpc_client: RpcClient,
    price_accounts: HashMap<String, Pubkey>,
}

#[derive(Debug, Clone)]
pub struct TokenPair {
    base: String,
    quote: String,
}

impl TokenPair {
    pub fn new(base: &str, quote: &str) -> Self {
        Self {
            base: base.to_string(),
            quote: quote.to_string(),
        }
    }
}

pub struct PriceData {
    prices: HashMap<String, f64>,
    history: HashMap<String, Vec<f64>>,
}

impl PriceFeed {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
        
        // Инициализация известных Pyth price accounts
        let mut price_accounts = HashMap::new();
        price_accounts.insert(
            "SOL/USD".to_string(),
            "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG".parse()?,
        );
        
        Ok(Self {
            rpc_client,
            price_accounts,
        })
    }

    pub async fn get_latest_prices(&self) -> Result<PriceData, Box<dyn Error>> {
        let mut prices = HashMap::new();
        let mut history = HashMap::new();

        for (pair, account) in &self.price_accounts {
            let data = self.rpc_client.get_account_data(account)?;
            let price_account: PriceAccount = bytemuck::from_bytes(&data);
            
            let price = price_account.agg.price as f64 / 10f64.powi(price_account.expo);
            prices.insert(pair.clone(), price);
            
            // Сохраняем историю цен
            history.entry(pair.clone())
                .or_insert_with(Vec::new)
                .push(price);
        }

        Ok(PriceData { prices, history })
    }
}

impl PriceData {
    pub fn get_price(&self, pair: &TokenPair) -> Result<f64, Box<dyn Error>> {
        let key = format!("{}/{}", pair.base, pair.quote);
        self.prices.get(&key)
            .copied()
            .ok_or_else(|| "Price not found".into())
    }

    pub fn get_average_price(&self, pair: &TokenPair, periods: usize) -> Result<f64, Box<dyn Error>> {
        let key = format!("{}/{}", pair.base, pair.quote);
        let history = self.history.get(&key)
            .ok_or_else(|| "Price history not found".into())?;
        
        let recent_prices = history.iter()
            .rev()
            .take(periods)
            .copied()
            .collect::<Vec<_>>();
            
        Ok(recent_prices.iter().sum::<f64>() / recent_prices.len() as f64)
    }
} 