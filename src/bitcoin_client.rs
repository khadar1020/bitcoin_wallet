use bitcoin::{
    secp256k1::{Secp256k1, KeyPair},
    Address, Network, PublicKey, XOnlyPublicKey
};
use rand::thread_rng;

pub fn generate_wallet() -> Result<(), Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    let network = Network::Testnet;

    let (secret_key, secp_public_key) = secp.generate_keypair(&mut thread_rng());
    
    let key_pair = KeyPair::from_secret_key(&secp, &secret_key);
    let (x_only_public_key, _parity) = XOnlyPublicKey::from_keypair(&key_pair);

    let tap_address = Address::p2tr(&secp, x_only_public_key, None, network);

    let bitcoin_pk = PublicKey::from_slice(&secp_public_key.serialize())?;
    let p2wpkh_address = Address::p2wpkh(&bitcoin_pk, network)?;

    println!("Pay to Witness Public Key Hash (P2WPKH): {}", 
             p2wpkh_address.to_qr_uri().to_ascii_lowercase());
    println!("Pay to Taproot (P2TR): {}", 
             tap_address.to_qr_uri().to_ascii_lowercase());

    Ok(())
}