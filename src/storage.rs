use std::fs;
use std::path::Path;

use crate::wallet::Wallet;

const FILE_PATH: &str = "wallet.json";


pub fn load_wallet() -> Option<Wallet> {
    if !Path::new(FILE_PATH).exists() {
        return None;
    }

    let data = fs::read_to_string(FILE_PATH).ok()?;
    let wallet: Wallet = serde_json::from_str(&data).ok()?;

    Some(wallet)
}


pub fn save_wallet(wallet: &Wallet) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(wallet)
        .expect("Error serializando la billetera");

    fs::write(FILE_PATH, json)
}
