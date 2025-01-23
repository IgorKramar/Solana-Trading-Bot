use anchor_client::{Client, Cluster};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
};
use std::rc::Rc;
use dotenv::dotenv;
use std::env;

mod trading_strategy;
mod price_feed;
mod jito_integration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let cluster_url = env::var("SOLANA_CLUSTER_URL")
        .expect("SOLANA_CLUSTER_URL must be set");
    let program_id = env::var("PROGRAM_ID")
        .expect("PROGRAM_ID must be set");
    let bot_private_key = env::var("BOT_PRIVATE_KEY")
        .expect("BOT_PRIVATE_KEY must be set");
    
    let payer = Keypair::new();
    let client = Client::new_with_options(
        Cluster::Devnet,
        Rc::new(payer),
        CommitmentConfig::confirmed(),
    );

    let program = client.program(trading_program::ID);
    
    // Инициализация компонентов бота
    let price_feed = price_feed::PriceFeed::new().await?;
    let trading_strategy = trading_strategy::TradingStrategy::new(program.clone());
    let jito_client = jito_integration::JitoClient::new().await?;

    println!("Торговый бот запущен");

    // Основной цикл бота
    loop {
        // Получаем текущие цены
        let price_data = price_feed.get_latest_prices().await?;
        
        // Анализируем рынок и генерируем сигналы
        let signals = trading_strategy.analyze_market(&price_data).await?;
        
        // Исполняем сигналы через Jito MEV
        for signal in signals {
            jito_client.execute_trade(signal).await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
} 