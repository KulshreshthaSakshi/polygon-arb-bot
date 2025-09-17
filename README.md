# Polygon Arbitrage Opportunity Detector Bot

A small Rust bot that **simulates** arbitrage between two UniswapV2-style DEX routers on the Polygon network (e.g., QuickSwap & SushiSwap).  
This project is read-only by default and **does not** perform any live trades — it only queries `getAmountsOut` to estimate potential profit and (optionally) logs results.

---

## Status
- Project created and running locally (read-only price queries).
- Basic price-compare logic implemented using `ethers-rs`.

---

## Prerequisites (on Windows 11)
- Rust + Cargo (install via https://rust-lang.org/tools/install)
- Git (https://git-scm.com/)
- MySQL server (optional — only if you want to log results to a database)
- An RPC endpoint for Polygon (public: `https://polygon-rpc.com` or private providers like Alchemy/QuickNode)

---

## Project structure
