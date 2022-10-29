mod communication;
mod domain;
mod strategies;

use domain::{DollarsPerBitcoin, Actor};
use log::info;
use strategies::create_strategies;
use communication::{get_bitcoin_value_usd, init_accounts, send_transactions};

use crate::domain::TradeRequest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .build();
    let strategies = create_strategies();
    let mut actors: Vec<_> = init_accounts(strategies.len() as u32)
        .await?
        .into_iter()
        .zip(strategies.into_iter())
        .map(|(init_resp, strategy)| Actor::new(init_resp.id, strategy, init_resp.wallet))
        .collect();
    loop {
        let current_btc = DollarsPerBitcoin::from(get_bitcoin_value_usd().await?);
        info!("Current BTC: {:?}", current_btc);
        let trade_requests = actors
            .iter_mut()
            .filter_map(|a| a.strategy.apply(&a.wallet, current_btc).map(|r| TradeRequest::new(a.id, r)))
            .collect();
        let new_wallets = send_transactions(trade_requests)
            .await?;
        for new_wallet in new_wallets {
            if let Some(actor) = actors.iter_mut().find(|a| a.id == new_wallet.actor_id) {
                actor.wallet = new_wallet.wallet;
            } else {
                log::error!("Received wallet that doesn't belong to any actor. Id: {}", new_wallet.actor_id);
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
