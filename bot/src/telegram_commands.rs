use teloxide::utils::command::BotCommands;
use serde::{Deserialize, Serialize};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    #[command(description = "Показать текущие позиции")]
    Positions,
    #[command(description = "Установить параметры торговой стратегии")]
    SetStrategy { name: String, params: String },
    #[command(description = "Остановить торговлю")]
    Stop,
    #[command(description = "Возобновить торговлю")]
    Start,
    #[command(description = "Показать статистику")]
    Stats,
    #[command(description = "Установить оповещения")]
    SetAlert { pair: String, price: f64 },
    #[command(description = "Показать балансы")]
    Balances,
    #[command(description = "Помощь")]
    Help,
}

#[derive(Serialize, Deserialize)]
pub struct AlertConfig {
    pub pair: String,
    pub price: f64,
    pub direction: AlertDirection,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize)]
pub enum AlertDirection {
    Above,
    Below,
}

impl Command {
    pub async fn handle(&self, ctx: CommandContext) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Command::Positions => {
                let positions = ctx.trading_strategy.get_positions().await?;
                let message = format_positions(positions);
                ctx.reply(message).await?;
            }
            Command::SetStrategy { name, params } => {
                ctx.trading_strategy.update_config(name, params).await?;
                ctx.reply("Стратегия обновлена").await?;
            }
            Command::Stop => {
                ctx.trading_strategy.stop().await?;
                ctx.reply("Торговля остановлена").await?;
            }
            Command::Start => {
                ctx.trading_strategy.start().await?;
                ctx.reply("Торговля запущена").await?;
            }
            Command::Stats => {
                let stats = ctx.monitor.get_stats().await?;
                let message = format_stats(stats);
                ctx.reply(message).await?;
            }
            Command::SetAlert { pair, price } => {
                ctx.alerts.add_alert(AlertConfig {
                    pair: pair.clone(),
                    price: *price,
                    direction: AlertDirection::Above,
                    user_id: ctx.user_id,
                }).await?;
                ctx.reply("Оповещение установлено").await?;
            }
            Command::Balances => {
                let balances = ctx.trading_strategy.get_balances().await?;
                let message = format_balances(balances);
                ctx.reply(message).await?;
            }
            Command::Help => {
                ctx.reply(Command::descriptions()).await?;
            }
        }
        Ok(())
    }
}

fn format_positions(positions: Vec<Position>) -> String {
    let mut message = String::from("📊 Текущие позиции:\n\n");
    for pos in positions {
        message.push_str(&format!(
            "🔸 {}: {} @ ${:.2} (PnL: ${:.2})\n",
            pos.pair, pos.size, pos.entry_price, pos.unrealized_pnl
        ));
    }
    message
}

fn format_stats(stats: Statistics) -> String {
    format!(
        "📈 Статистика торговли:\n\n\
        Всего сделок: {}\n\
        Успешных: {}%\n\
        Общий P&L: ${:.2}\n\
        Средний P&L на сделку: ${:.2}\n\
        Время работы: {}h",
        stats.total_trades,
        stats.win_rate,
        stats.total_pnl,
        stats.avg_trade_pnl,
        stats.uptime_hours
    )
} 