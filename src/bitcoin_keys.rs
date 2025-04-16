use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::Network;
use rand::RngCore;
use hex;

pub struct BitcoinKeys {
    pub master_key: String,
    pub network: Network,
}

impl BitcoinKeys {
    pub fn new(secret_seed: Option<String>) -> Self {
        let network = Network::Testnet;
        let seed_bytes = match secret_seed {
            Some(secret) => hex::decode(secret).expect("Invalid hex seed"),
            None => {
                let mut bytes = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut bytes);
                bytes.to_vec()
            }
        };

        let master_key = ExtendedPrivKey::new_master(network, &seed_bytes)
            .expect("Failed to create master key");

        BitcoinKeys {
            master_key: master_key.to_string(),
            network,
        }
    }
}