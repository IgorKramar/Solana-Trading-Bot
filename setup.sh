#!/bin/bash

# Проверка наличия необходимых инструментов
command -v node >/dev/null 2>&1 || { echo "Node.js не установлен" >&2; exit 1; }
command -v yarn >/dev/null 2>&1 || { echo "Yarn не установлен" >&2; exit 1; }
command -v rustc >/dev/null 2>&1 || { echo "Rust не установлен" >&2; exit 1; }

# Установка Solana CLI и Anchor
curl -sSfL https://release.solana.com/v1.14.0/install | sh
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked

# Создание проекта
anchor init solana-trading-bot
cd solana-trading-bot

# Создание структуры директорий
mkdir -p {programs/trading-program/src,app/src/{components,hooks,types,providers},bot/src/{strategies,utils}}

# Копирование файлов конфигурации
# package.json уже создан в app/package.json
cd app
yarn install

# Установка дополнительных зависимостей
yarn add \
    @solana/wallet-adapter-base \
    @solana/wallet-adapter-react-ui \
    @solana/wallet-adapter-wallets \
    @solana/web3.js \
    @project-serum/anchor \
    chart.js \
    react-chartjs-2 \
    tailwindcss \
    postcss \
    autoprefixer

# Инициализация Tailwind CSS
npx tailwindcss init -p

cd ..

# Настройка Rust бота
cd bot
cargo init
cargo add \
    anchor-client \
    solana-sdk \
    solana-client \
    tokio --features full \
    serde --features derive \
    serde_json \
    teloxide --features macros \
    pyth-sdk-solana \
    bytemuck \
    log \
    env_logger \
    dotenv

cd ..

# Создание .env
cat > .env << EOL
TELEGRAM_BOT_TOKEN=your_bot_token
RPC_URL=https://api.devnet.solana.com
PROGRAM_ID=your_program_id
EOL

# Создание .gitignore
cat > .gitignore << EOL
# Dependencies
node_modules/
target/

# Build outputs
dist/
build/
.anchor/

# Environment
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# IDE
.idea/
.vscode/
*.swp
*.swo

# Logs
npm-debug.log*
yarn-debug.log*
yarn-error.log*
*.log

# System Files
.DS_Store
Thumbs.db
EOL

# Генерация ключей для Solana
solana-keygen new --no-bip39-passphrase -o id.json

# Установка переменных окружения Solana
solana config set --url https://api.devnet.solana.com
solana config set --keypair ./id.json

# Получение тестовых SOL
solana airdrop 2

echo "Установка завершена! Проверьте README.md для дальнейших инструкций."
echo "Не забудьте настроить переменные окружения в .env файле"
echo "Для запуска веб-интерфейса: cd app && yarn start"
echo "Для запуска бота: cd bot && cargo run" 