use log::{info, warn, error};
use std::time::{Duration, Instant};
use tokio::time::interval;

pub struct PerformanceMetrics {
    pub trades_executed: u64,
    pub successful_trades: u64,
    pub failed_trades: u64,
    pub average_execution_time: Duration,
    pub total_profit_loss: f64,
}

pub struct Monitor {
    start_time: Instant,
    metrics: PerformanceMetrics,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            metrics: PerformanceMetrics {
                trades_executed: 0,
                successful_trades: 0,
                failed_trades: 0,
                average_execution_time: Duration::new(0, 0),
                total_profit_loss: 0.0,
            },
        }
    }

    pub fn record_trade(&mut self, success: bool, execution_time: Duration, profit_loss: f64) {
        self.metrics.trades_executed += 1;
        if success {
            self.metrics.successful_trades += 1;
        } else {
            self.metrics.failed_trades += 1;
        }

        // Обновление среднего времени исполнения
        let total_trades = self.metrics.trades_executed as u32;
        self.metrics.average_execution_time = (self.metrics.average_execution_time * (total_trades - 1) + execution_time) / total_trades;
        
        self.metrics.total_profit_loss += profit_loss;

        info!(
            "Trade executed: Success={}, Time={:?}, PnL={}, Total PnL={}",
            success, execution_time, profit_loss, self.metrics.total_profit_loss
        );
    }

    pub async fn start_monitoring(&self) {
        let mut interval = interval(Duration::from_secs(300)); // каждые 5 минут

        loop {
            interval.tick().await;
            self.report_metrics();
        }
    }

    fn report_metrics(&self) {
        let uptime = self.start_time.elapsed();
        info!("=== Performance Report ===");
        info!("Uptime: {:?}", uptime);
        info!("Total trades: {}", self.metrics.trades_executed);
        info!("Success rate: {:.2}%", 
            (self.metrics.successful_trades as f64 / self.metrics.trades_executed as f64) * 100.0
        );
        info!("Average execution time: {:?}", self.metrics.average_execution_time);
        info!("Total PnL: {:.2} USDC", self.metrics.total_profit_loss);
        info!("========================");
    }
} 