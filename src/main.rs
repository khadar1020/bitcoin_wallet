pub mod bitcoin_client;
pub mod client_wallet;
pub mod bitcoin_keys;

use client_wallet::WalletContext;

fn main() {
    let from = "Enter your from address here";
    let wallet = WalletContext::new(None);
    wallet.get_balance();
    let to = "Enter your to address here";
    wallet.send_coins(to, 100);
}