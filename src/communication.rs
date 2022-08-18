use anyhow::{anyhow, bail, Result};

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
