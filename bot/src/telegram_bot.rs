use teloxide::{prelude::*, utils::command::BotCommands};
use std::sync::Arc;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Показать текущие позиции")]
    Positions,
    #[command(description = "Установить параметры торговой стратегии")]
    SetStrategy { name: String, params: String },
    #[command(description = "Остановить торговлю")]
    Stop,
    #[command(description = "Возобновить торговлю")]
    Start,
}

pub struct TelegramBot {
    bot: Bot,
    trading_strategy: Arc<trading_strategy::TradingStrategy>,
}

impl TelegramBot {
    pub async fn new(trading_strategy: Arc<trading_strategy::TradingStrategy>) -> Result<Self, Box<dyn std::error::Error>> {
        let bot = Bot::from_env();
        Ok(Self { bot, trading_strategy })
    }

    pub async fn run(self) {
        let handler = Update::filter_message()
            .filter_command::<Command>()
            .endpoint(|msg: Message, bot: Bot, cmd: Command| async move {
                match cmd {
                    Command::Positions => {
                        // Получение и отображение текущих позиций
                    }
                    Command::SetStrategy { name, params } => {
                        // Обновление параметров стратегии
                    }
                    Command::Stop => {
                        // Остановка торговли
                    }
                    Command::Start => {
                        // Запуск торговли
                    }
                };
                Ok(())
            });

        Dispatcher::builder(self.bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
} 