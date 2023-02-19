use ethers::prelude::*;
use ethers_providers::{Provider, Http, Middleware};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::env;
use std::fmt::LowerHex;
use std::ptr::null;

// async fn get_provider(provider: &str, key: &str) -> Option<Provider>{

//     let prov = provider.to_lowercase().as_str();

//     let url = match prov {
//         "infura" => format!("https://mainnet.infura.io/v3/{}", key).to_string(),
//         _ => None 
//     };

//     match url {
//         Some(T) => Provider::<Http>::try_from(url),
//         _ => None
//     }

    
// }

// async fn get_balance(addr: &str, block_no: Option<)

#[tokio::main]
async fn main() {
    let infura_api = env::var("INFURA_API").unwrap();

    let provider = Provider::<Http>::try_from(
        format!("https://mainnet.infura.io/v3/{infura_api}")
    ).unwrap();

    let null_add = "0x0000000000000000000000000000000000000000";

    // let provider = get_provider("Infura", &infura_api).await.unwrap();


    let last_block = provider.get_block_number().await.unwrap();

    for i in (49164..last_block.as_u64()).step_by(1000) {
        let block_no = ethers_core::types::BlockId::from(i);
        let null_bal = provider.get_balance(null_add, Some(block_no)).await;
        
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