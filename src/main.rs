// src/main.rs
use std::env;
use std::sync::Arc;

use anyhow::Result;
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::U256;
use tokio::time::{sleep, Duration};

abigen!(
    UniswapV2Router,
    r#"[
        function getAmountsOut(uint256 amountIn, address[] memory path) external view returns (uint256[] memory amounts)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
    // load .env
    dotenv().ok();

    // read RPC URL from .env
    let rpc_url = env::var("RPC_URL")
        .expect("RPC_URL must be set in .env (e.g. https://polygon-rpc.com)");

    // connect to provider and wrap in Arc
    let provider = Provider::<Http>::try_from(rpc_url.as_str())
        .expect("Failed to create provider");
    let provider = Arc::new(provider);

    println!("✅ Connected to RPC provider");

    // DEX router addresses (replace with Polygon addresses in your .env if you prefer)
    // Example placeholders - replace with values from your .env if you want dynamic config
    let dex1_addr: Address = env::var("DEX1_ROUTER")
        .expect("DEX1_ROUTER must be set in .env")
        .parse()?;
    let dex2_addr: Address = env::var("DEX2_ROUTER")
        .expect("DEX2_ROUTER must be set in .env")
        .parse()?;

    // instantiate typed router bindings (expects Arc<Provider<..>>)
    let router1 = UniswapV2Router::new(dex1_addr, provider.clone());
    let router2 = UniswapV2Router::new(dex2_addr, provider.clone());

    println!("✅ Router clients initialized");

    // token addresses from .env
    let usdc: Address = env::var("USDC_ADDRESS")
        .expect("USDC_ADDRESS must be set in .env")
        .parse()?;
    let weth: Address = env::var("WETH_ADDRESS")
        .expect("WETH_ADDRESS must be set in .env")
        .parse()?;

    // Example: simulate 1000 USDC trade (USDC has 6 decimals on Polygon)
    let trade_usdc: u64 = env::var("TRADE_USDC")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);
    let amount_in: U256 = U256::from(trade_usdc) * U256::from(1_000_000u64); // trade_usdc * 10^6

    // Build path: USDC -> WETH
    let path_usdc_weth = vec![usdc, weth];

    // Query router1: USDC -> WETH
    let amounts_out1 = router1
        .get_amounts_out(amount_in, path_usdc_weth.clone())
        .call()
        .await?;
    let weth_received = amounts_out1.last().cloned().unwrap_or_default();

    // Query router2: WETH -> USDC (sell on router2)
    let path_weth_usdc = vec![weth, usdc];
    let amounts_out2 = router2
        .get_amounts_out(weth_received, path_weth_usdc.clone())
        .call()
        .await?;
    let usdc_after = amounts_out2.last().cloned().unwrap_or_default();

    // convert U256 -> f64 for printing (note: careful with precision; this is for display)
    let in_usdc_f = (amount_in.as_u128() as f64) / 1e6f64;
    let out_usdc_f = (usdc_after.as_u128() as f64) / 1e6f64;
    let profit = out_usdc_f - in_usdc_f;

    println!("Simulated trade: in ${:.6} -> out ${:.6} => profit ${:.6}", in_usdc_f, out_usdc_f, profit);

    // keep program alive briefly if you want to loop later (example sleep)
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
