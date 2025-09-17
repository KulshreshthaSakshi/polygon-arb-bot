# Polygon Arbitrage Opportunity Bot 

This project detects arbitrage opportunities between decentralized exchanges (DEXes) on the Polygon network.  
It compares token swap prices across DEX routers (e.g., QuickSwap and SushiSwap), simulates trades, and logs profitable opportunities in a MySQL database.

---

## Features 

- Connects to **Polygon RPC** in real time  
- Loads **DEX router ABIs** and interacts with contracts using `ethers-rs`  
- Detects price discrepancies between QuickSwap & SushiSwap    
- Configurable via simple `.env` file (RPC URL, token addresses, DB credentials)  
- Prints **live updates** to the console  

---

## Setup & Installation ⚙

### 1. Clone the repository

```bash
git clone <your-repo-url>
cd polygon-arb-bot
```
2. Install Rust

Follow the official Rust installation guide
.

3. Build dependencies
cargo build

Configure Environment 🌍

Create a .env file in the project root with:

# Polygon RPC (public or provider endpoint)
RPC_URL=https://polygon-rpc.com

# DEX routers on Polygon
DEX1_ROUTER=0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff   # QuickSwap
DEX2_ROUTER=0x1b02da8cb0d097eb8d57a175b88c7d8b47997506   # SushiSwap

# Token addresses on Polygon
USDC_ADDRESS=0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174
WETH_ADDRESS=0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619

# MySQL connection string
MYSQL_URL=mysql://root:yourpassword@127.0.0.1:3306/polygon_arb

# Bot parameters
TRADE_USDC=1000
MIN_PROFIT_USDC=1.0
SIMULATED_GAS_USDC=0.5
POLL_INTERVAL_SECS=10


⚠️ Do not commit .env to GitHub. Add it to .gitignore.

Database Setup 🗄️

Login to MySQL and create a database:

CREATE DATABASE polygon_arb;
USE polygon_arb;


Create the table for storing opportunities:

CREATE TABLE arbitrage_opportunities (
    id INT AUTO_INCREMENT PRIMARY KEY,
    ts TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    token_pair VARCHAR(32),
    dex_buy VARCHAR(128),
    dex_sell VARCHAR(128),
    amount_in_usdc DECIMAL(36,6),
    amount_out_usdc DECIMAL(36,6),
    profit_usdc DECIMAL(36,6),
    gas_usdc DECIMAL(36,6),
    logged_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

Running the Bot ▶️

Run in development mode:

cargo run


This will:

Connect to Polygon RPC

Fetch swap quotes from both DEX routers

Compare simulated trades

Detect arbitrage opportunities

Store them in MySQL

Print results in console

Example output:

[INFO] Checking pair: USDC → WETH
[INFO] QuickSwap: 1000 USDC → 0.6021 WETH
[INFO] SushiSwap: 1000 USDC → 0.6032 WETH
[ARB] Opportunity! Buy on QuickSwap, Sell on SushiSwap → Profit = 0.0011 WETH (~1.2 USDC)

Example Database Entry 📊
id	ts	token_pair	dex_buy	dex_sell	amount_in_usdc	amount_out_usdc	profit_usdc	gas_usdc	logged_at
1	2025-09-17 14:35:10	USDC/WETH	QuickSwap	SushiSwap	1000.000000	1001.200000	1.200000	0.500000	2025-09-17 14:35:10
Scalability & Future Improvements 🚀

More exchanges – Add additional DEX routers for deeper coverage

Additional tokens – Track multiple token pairs simultaneously

Automated trading – Extend bot to execute swaps (requires wallets & gas management)

Dashboard – Web UI with charts & analytics

Resilience – Add retry logic, caching, better error handling

Tech Stack 🛠️

Rust – Core bot logic

ethers-rs – Smart contract interaction

dotenvy – Load .env config

tokio – Async runtime

serde – JSON parsing

MySQL – Storage layer
