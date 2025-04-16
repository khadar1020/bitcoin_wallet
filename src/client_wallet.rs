use bdk::{
    blockchain::{Blockchain, ElectrumBlockchain},
    database::MemoryDatabase,
    descriptor::ExtendedDescriptor,
    wallet::{AddressIndex, Wallet},
    FeeRate, SignOptions, SyncOptions,
};
use bdk::bitcoin::{Address, Network};
use bdk::electrum_client::Client;
use std::str::FromStr;

pub struct WalletContext {
    wallet_state: Wallet<MemoryDatabase>,
    blockchain: ElectrumBlockchain,
}

impl WalletContext {
    pub fn new(seed: Option<String>) -> Self {
        let key = crate::bitcoin_keys::BitcoinKeys::new(seed);
        let master_key = bdk::bitcoin::util::bip32::ExtendedPrivKey::from_str(&key.master_key).unwrap();

        let descriptor_str = format!("wpkh({}/84h/0h/0h/0/*)", master_key);
        let descriptor = ExtendedDescriptor::from_str(&descriptor_str).unwrap();

        let wallet_state = Wallet::new(
            descriptor,
            None,
            Network::Testnet,
            MemoryDatabase::default(),
        )
        .unwrap();

        let blockchain = ElectrumBlockchain::from(
            Client::new("ssl://electrum.blockstream.info:60002").unwrap()
        );

        WalletContext { wallet_state, blockchain }
    }

    pub fn get_balance(&self) {
        self.wallet_state.sync(&self.blockchain, SyncOptions::default()).unwrap();
        let receive_address = self.wallet_state.get_address(AddressIndex::LastUnused).unwrap();
        let balance = self.wallet_state.get_balance().unwrap();
        println!("Bitcoin address: {}", receive_address.address);
        println!("Balance: {} sats", balance);
    }

    pub fn send_coins(&self, send_address: &str, sats: u64) {
        self.wallet_state.sync(&self.blockchain, SyncOptions::default()).unwrap();
        let address = Address::from_str(send_address).unwrap();
        if address.network != Network::Testnet {
            panic!("Address is not for Testnet");
        }

        let mut builder = self.wallet_state.build_tx();
        builder
            .drain_wallet().fee_rate(FeeRate::from_sat_per_vb(2.0)).drain_to(address.script_pubkey());

        let (mut psbt, _) = builder.finish().unwrap();
        self.wallet_state.sign(&mut psbt, SignOptions::default()).unwrap();
        
        let tx = psbt.extract_tx();
        self.blockchain.broadcast(&tx).unwrap();
        println!("Broadcasted TX ID: {}", tx.txid());
    }
}