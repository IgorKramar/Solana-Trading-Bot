
# Solana Trading Bot

Децентрализованное приложение для автоматизированной торговли на Solana с интеграцией Telegram и веб-интерфейсом.

## 🚀 Основные возможности

- Автоматическая торговля на Serum DEX
- Интеграция с Jito MEV для оптимизации исполнения транзакций
- Управление через Telegram бота
- Веб-интерфейс для мониторинга и управления
- Настраиваемые торговые стратегии
- Интеграция с Pyth для получения цен

## 📋 Требования

- Node.js >= 16
- Rust >= 1.65
- Solana CLI
- Anchor Framework
- Yarn

## 🛠 Установка

1. **Установите зависимости**

```bash
# Установка Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Установка Solana
sh -c "$(curl -sSfL https://release.solana.com/v1.14.0/install)"

# Установка Anchor
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked

# Установка зависимостей Node.js
cd app && yarn install
```

2. **Настройте переменные окружения**

Создайте файл `.env` в корневой директории:

```env
TELEGRAM_BOT_TOKEN=your_bot_token
RPC_URL=https://api.devnet.solana.com
PROGRAM_ID=your_program_id
```

3. **Настройте Solana**

```bash
solana-keygen new --no-bip39-passphrase
solana config set --url https://api.devnet.solana.com
solana airdrop 2 # для тестирования на devnet
```

## 🚀 Запуск

1. **Запуск смарт-контракта**

```bash
cd programs/trading-program
anchor build
anchor deploy
```

2. **Запуск веб-интерфейса**

```bash
cd app
yarn start
```

3. **Запуск торгового бота**

```bash
cd bot
cargo run
```

## 📁 Структура проекта

```
solana-trading-bot/
├── app/                    # React веб-приложение
│   ├── src/
│   │   ├── components/    # React компоненты
│   │   ├── hooks/        # Custom React hooks
│   │   ├── providers/    # React контекст провайдеры
│   │   └── types/        # TypeScript типы
│   
├── bot/                    # Rust торговый бот
│   ├── src/
│   │   ├── strategies/   # Торговые стратегии
│   │   ├── utils/        # Вспомогательные функции
│   │   └── main.rs       # Точка входа бота
│   
└── programs/              # Solana смарт-контракты
    └── trading-program/
        └── src/
            └── lib.rs     # Основной смарт-контракт
```

## 💡 Использование

### Веб-интерфейс

1. Подключите кошелек Phantom
2. Выберите торговую стратегию
3. Настройте параметры
4. Запустите торговлю

### Telegram бот

Доступные команды:
- `/start` - Начать работу с ботом
- `/positions` - Показать текущие позиции
- `/stats` - Показать статистику
- `/strategy` - Настроить стратегию
- `/balance` - Показать баланс

## 🔒 Безопасность

- Все приватные ключи хранятся локально
- Используется шифрование для коммуникации с Telegram
- Реализована защита от несанкционированного доступа
- Поддержка multisig для критических операций

## 🤝 Вклад в развитие

Мы приветствуем вклад в развитие проекта! Пожалуйста:

1. Форкните репозиторий
2. Создайте ветку для ваших изменений
3. Внесите изменения
4. Создайте Pull Request

## 📝 Лицензия

MIT License. См. [LICENSE](LICENSE.md) для деталей.

## ⚠️ Дисклеймер

Это экспериментальное программное обеспечение. Используйте на свой страх и риск. Авторы не несут ответственности за возможные потери.