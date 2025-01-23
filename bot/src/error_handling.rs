use std::error::Error;
use std::fmt;
use tokio::time::{sleep, Duration};
use log::{error, warn, info};

#[derive(Debug)]
pub enum TradingError {
    NetworkError(String),
    InsufficientFunds(String),
    InvalidOrder(String),
    RpcError(String),
    ExecutionError(String),
}

impl fmt::Display for TradingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradingError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            TradingError::InsufficientFunds(msg) => write!(f, "Insufficient funds: {}", msg),
            TradingError::InvalidOrder(msg) => write!(f, "Invalid order: {}", msg),
            TradingError::RpcError(msg) => write!(f, "RPC error: {}", msg),
            TradingError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
        }
    }
}

impl Error for TradingError {}

pub async fn retry_with_backoff<T, F, Fut>(
    operation: F,
    max_retries: u32,
    initial_delay: Duration,
) -> Result<T, Box<dyn Error>>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, Box<dyn Error>>>,
{
    let mut retries = 0;
    let mut delay = initial_delay;

    loop {
        match operation().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                if retries >= max_retries {
                    return Err(e);
                }

                warn!("Operation failed: {}. Retrying in {:?}...", e, delay);
                sleep(delay).await;
                
                retries += 1;
                delay *= 2; // Экспоненциальный backoff
            }
        }
    }
}

pub struct ErrorHandler {
    monitor: Monitor,
}

impl ErrorHandler {
    pub fn new(monitor: Monitor) -> Self {
        Self { monitor }
    }

    pub async fn handle_error(&self, error: Box<dyn Error>) {
        match error.downcast_ref::<TradingError>() {
            Some(TradingError::NetworkError(_)) => {
                self.handle_network_error(error).await;
            }
            Some(TradingError::InsufficientFunds(_)) => {
                self.handle_insufficient_funds(error).await;
            }
            Some(TradingError::InvalidOrder(_)) => {
                self.handle_invalid_order(error).await;
            }
            _ => {
                error!("Unexpected error: {}", error);
                self.monitor.record_error(error);
            }
        }
    }

    async fn handle_network_error(&self, error: Box<dyn Error>) {
        error!("Network error occurred: {}", error);
        self.monitor.record_error(error);
        
        // Попытка переподключения
        retry_with_backoff(
            || async { self.reconnect().await },
            5,
            Duration::from_secs(1),
        ).await.unwrap_or_else(|e| {
            error!("Failed to reconnect: {}", e);
        });
    }

    async fn handle_insufficient_funds(&self, error: Box<dyn Error>) {
        error!("Insufficient funds: {}", error);
        self.monitor.record_error(error);
        
        // Отмена всех активных ордеров
        if let Err(e) = self.cancel_all_orders().await {
            error!("Failed to cancel orders: {}", e);
        }
    }

    async fn handle_invalid_order(&self, error: Box<dyn Error>) {
        error!("Invalid order: {}", error);
        self.monitor.record_error(error);
    }

    async fn reconnect(&self) -> Result<(), Box<dyn Error>> {
        // Логика переподключения
        Ok(())
    }

    async fn cancel_all_orders(&self) -> Result<(), Box<dyn Error>> {
        // Логика отмены всех ордеров
        Ok(())
    }
} 