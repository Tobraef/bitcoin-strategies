use anyhow::{anyhow, bail, Result};

use crate::domain::{TradeRequest, InitResponse, TradeResponse};

pub async fn get_bitcoin_value_usd() -> Result<f32> {
    let response = reqwest::get("https://blockchain.info/ticker")
        .await?
        .text()
        .await?;
    let map: serde_json::Map<_, _> = serde_json::from_str(&response)?;
    let usd = map
        .get("USD")
        .ok_or(anyhow!("Didn't find USD node in response."))?;
    let last = usd.get("last").ok_or(anyhow!(
        "Didn't find 'last' in USD node. Node was: {:?}",
        usd
    ))?;
    match last {
        serde_json::Value::Number(n) => {
            let value = n
                .as_f64()
                .ok_or(anyhow!("Last number value is not a float."))?;
            Ok(value as f32)
        }
        _ => bail!("Last has invalid type. Expected number, was: {:?}", last),
    }
}

fn port_url() -> u16 {
    std::env::var("BANK_PORT")
        .and_then(|port_str| port_str.parse().map_err(|_| std::env::VarError::NotPresent))
        .unwrap_or(8765)
}

fn bank_url() -> String {
    format!("http://localhost:{}", port_url())
}

pub async fn init_accounts(count: u32) -> Result<Vec<InitResponse>> {
    let endpoint = format!("{}/init-accounts/{count}", bank_url());
    reqwest::get(endpoint)
        .await?
        .json()
        .await
        .map_err(|e| anyhow!("Failed sending init accounts: {e}"))
}

pub async fn send_transactions(transactions: Vec<TradeRequest>) -> Result<Vec<TradeResponse>> {
    let endpoint = format!("{}/trade-request", bank_url());
    let request = reqwest::Client::new()
        .post(endpoint)
        .body(serde_json::to_string(&transactions)?);
    request
        .send()
        .await
        .map_err(|e| anyhow!("Failed sending trade requests: {e}"))?
        .json()
        .await
        .map_err(|e| anyhow!("Failed parsing wallets: {e}"))
}
