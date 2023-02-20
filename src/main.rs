use ethers::prelude::*;
use ethers_providers::{Provider, Http, Middleware};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::env;
use std::fmt::LowerHex;

async fn get_provider(provider: &str, key: &str) -> Option<Provider<Http>>{

    let prov = provider.to_lowercase();

    let url = match prov.as_str() {
        "infura" => {
            let path = format!("https://mainnet.infura.io/v3/{}", key).to_string();
            Provider::<Http>::try_from(path)
        },
        _ => 
    };

    match url {
        Ok(provider) => Some(provider),
        Err(_) => None
    }

    
}

// async fn get_balance(addr: &str, block_no: Option<)

#[tokio::main]
async fn main() {
    let infura_api = env::var("INFURA_API").unwrap();

    let provider = Provider::<Http>::try_from(
        format!("https://mainnet.infura.io/v3/{infura_api}")
    ).unwrap();

    let null_add = "0x0000000000000000000000000000000000000000";
    let poloniex_add = "0x32Be343B94f860124dC4fEe278FDCBD38C102D88";

    // let provider = get_provider("Infura", &infura_api).await.unwrap();


    let last_block = provider.get_block_number().await.unwrap();

    for i in ((last_block.as_u64()-20)..last_block.as_u64()) {
        let block_no = ethers_core::types::BlockId::from(i);
        println!("{:?}", block_no);
        let null_bal = provider.get_balance(poloniex_add, None).await;
        println!("{:?}", null_bal);
        match null_bal {
            Ok(T) => {
                // let null_bal_res = null_bal.unwrap();
                println!("Balance at {}: {}", i, T)
            },
            Err(T) => {println!("Balance at {}: ?", i)}
        }
        
        
        
    }

    let block_res = provider.get_block(last_block).await;

    // provider.get_balance(from, block)

    // let block_res = provider.get_block("100u64").await;

    let block = block_res.unwrap();

    println!("Got block: {}", serde_json::to_string(&block).unwrap());

}