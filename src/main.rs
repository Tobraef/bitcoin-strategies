mod communication;
mod domain;
mod strategies;

use domain::{DollarsPerBitcoin, Wallet, Bitcoin, Dollar, change_balance, total_dollars};
use strategies::{create_strategies, Strategy};
use communication::get_bitcoin_value_usd;

struct Node {
    wallet: Wallet,
    strategy: Box<dyn Strategy>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wallet_factory = || {
        let mut w = Wallet::default();
        w.btc = Bitcoin::from(1.);
        w.dollars = Dollar::from(10_000.);
        w
    };
    let strategies = create_strategies();
    let mut nodes: Vec<_> = strategies
        .into_iter()
        .map(|s| Node { strategy: s, wallet: wallet_factory() })
        .collect();
    for _ in 0..10 {
        let current_btc = DollarsPerBitcoin::from(get_bitcoin_value_usd().await?);
        println!("Current BTC: {:?}", current_btc);
        for node in nodes.iter_mut() {
            if let Some(trade) = node.strategy.apply(&node.wallet, current_btc) {
                change_balance(&mut node.wallet, trade, current_btc);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(3600 / 4)).await;
    }
    let current_btc = DollarsPerBitcoin::from(get_bitcoin_value_usd().await?);

    for node in nodes {
        println!("{} ended up with {}$", node.strategy.to_string(), *total_dollars(&node.wallet, current_btc).as_ref());
    }
    Ok(())
}
