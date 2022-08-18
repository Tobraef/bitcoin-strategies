mod communication;
mod domain;
mod strategies;

use domain::DollarsPerBitcoin;
use strategies::create_strategies;
use communication::get_bitcoin_value_usd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let strategies = create_strategies();
    let current_btc = DollarsPerBitcoin::from(get_bitcoin_value_usd().await?);
    Ok(())
}
