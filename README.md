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

## Setup & Installation 

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

Configure Environment 

Create a .env file in the project root with:

# Polygon RPC (public or provider endpoint)
RPC_URL=https://polygon-rpc.com

# DEX routers on Polygon
DEX1_ROUTER=0xa5E0829CaCEd8fFDD4De3c43696c57F7D7A678ff   # QuickSwap
DEX2_ROUTER=0x1b02da8cb0d097eb8d57a175b88c7d8b47997506   # SushiSwap

# Token addresses on Polygon
USDC_ADDRESS=0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174
WETH_ADDRESS=0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619



# Bot parameters
TRADE_USDC=1000
MIN_PROFIT_USDC=1.0
SIMULATED_GAS_USDC=0.5
POLL_INTERVAL_SECS=10

# Running the Bot 

**Run in development mode:**

    cargo run


This will:

Connect to Polygon RPC

Fetch swap quotes from both DEX routers

Compare simulated trades

Detect arbitrage opportunities

Print results in console




# Scalability & Future Improvements
More exchanges – Add additional DEX routers for deeper coverage

Additional tokens – Track multiple token pairs simultaneously

Automated trading – Extend bot to execute swaps (requires wallets & gas management)

Dashboard – Web UI with charts & analytics

Resilience – Add retry logic, caching, better error handling

# Tech Stack 🛠️

Rust – Core bot logic

ethers-rs – Smart contract interaction

dotenvy – Load .env config

tokio – Async runtime

serde – JSON parsing


