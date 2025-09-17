
use std::env;
use std::sync::Arc;

use anyhow::Result;
use dotenv::dotenv;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::U256;
use tokio::time::{sleep, Duration};
use chrono::Local;

abigen!(
    UniswapV2Router,
    r#"[ 
        function getAmountsOut(uint256 amountIn, address[] memory path) external view returns (uint256[] memory amounts)
    ]"#,
);

#[tokio::main]
async fn main() -> Result<()> {
  
    dotenv().ok();

    
    let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set in .env");
    let provider = Provider::<Http>::try_from(rpc_url.as_str())
        .expect("Failed to create provider");
    let provider = Arc::new(provider);

    println!("✅ Connected to RPC provider");

    
    let dex1_addr: Address = env::var("DEX1_ROUTER")?.parse()?;
    let dex2_addr: Address = env::var("DEX2_ROUTER")?.parse()?;

    let router1 = UniswapV2Router::new(dex1_addr, provider.clone());
    let router2 = UniswapV2Router::new(dex2_addr, provider.clone());
    println!("✅ Router clients initialized");

   
    let usdc: Address = env::var("USDC_ADDRESS")?.parse()?;
    let weth: Address = env::var("WETH_ADDRESS")?.parse()?;

    
    let trade_usdc: u64 = env::var("TRADE_USDC")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);
    let amount_in: U256 = U256::from(trade_usdc) * U256::from(1_000_000u64);

   
    let min_profit: f64 = env::var("MIN_PROFIT_USDC")
        .unwrap_or_else(|_| "0.5".to_string())
        .parse()
        .unwrap_or(0.5);

    
    let simulated_gas_cost: f64 = env::var("SIMULATED_GAS_USDC")
        .unwrap_or_else(|_| "1.0".to_string())
        .parse()
        .unwrap_or(1.0);

    println!("✅ Bot configuration loaded");

    
    loop {
        if let Err(e) = check_arbitrage(&router1, &router2, amount_in, usdc, weth, min_profit, simulated_gas_cost).await {
            eprintln!("[{}] ❌ Error: {:?}", Local::now().format("%Y-%m-%d %H:%M:%S"), e);
        }
        sleep(Duration::from_secs(5)).await;
    }
}

async fn check_arbitrage(
    router1: &UniswapV2Router<Provider<Http>>,
    router2: &UniswapV2Router<Provider<Http>>,
    amount_in: U256,
    usdc: Address,
    weth: Address,
    min_profit: f64,
    simulated_gas: f64,
) -> Result<()> {
    
    let path_usdc_weth = vec![usdc, weth];
    let amounts_out1 = router1.get_amounts_out(amount_in, path_usdc_weth.clone())
        .call()
        .await?;
    let weth_received = amounts_out1.last().cloned().unwrap_or_default();

    
    let path_weth_usdc = vec![weth, usdc];
    let amounts_out2 = router2.get_amounts_out(weth_received, path_weth_usdc.clone())
        .call()
        .await?;
    let usdc_after = amounts_out2.last().cloned().unwrap_or_default();

    
    let in_usdc_f = amount_in.as_u128() as f64 / 1e6;
    let out_usdc_f = usdc_after.as_u128() as f64 / 1e6;
    let profit = out_usdc_f - in_usdc_f;
    let net_profit = profit - simulated_gas;

    
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    if net_profit >= min_profit {
        println!("[{}]  Arbitrage Opportunity Detected!", timestamp);
        println!("    Trade: ${:.2} -> ${:.2}", in_usdc_f, out_usdc_f);
        println!("    Simulated Gas Cost: ${:.2}", simulated_gas);
        println!("    Estimated Net Profit: ${:.2}", net_profit);
    } else {
        println!("[{}]  No profitable arbitrage. Potential Profit: ${:.2}", timestamp, net_profit);
    }

    Ok(())
}
