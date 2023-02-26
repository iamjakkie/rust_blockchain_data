use ethers::prelude::*;
use ethers_providers::{Provider, Http, Middleware, StreamExt, Ws};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::env;
use std::fmt::LowerHex;
use std::sync::Arc;

use ethers::{
    core::{
        abi::AbiDecode,
        types::{BlockNumber, Filter, U256},
    }
};

struct CustomError {
    pub message: String,
}

async fn get_provider(provider: &str, key: &str) -> Result<Provider<Http>, CustomError>{

    // let prov = provider.to_lowercase();

    // let url = match prov.as_str() {
    //     "infura" => {
    //         let path = format!("https://mainnet.infura.io/v3/{}", key).to_string();
    //         let res = Provider::<Http>::try_from(path)
    //             .map_err(|e| Err(format!("{}", e.to_string())));
    //         res
    //     },
    //     _ => Err(format!("Oh no")),
    // };

    // match url {
    //     Ok(provider) => Some(provider),
    //     Err(_) => None
    // }

    let prov = provider.to_lowercase();

    match prov.as_str() {
        "infura" => {
            let path = format!("https://mainnet.infura.io/v3/{}", key).to_string();
            Provider::<Http>::try_from(path)
                .map_err(|e| CustomError { message: e.to_string() })
        },
        _ => Err(CustomError { message: format!("Oh no") }),
    }
}

// async fn get_balance(addr: &str, block_no: Option<)

#[tokio::main]
async fn main() {
    let infura_api = env::var("INFURA_API").unwrap();

    let provider = Provider::<Http>::try_from(
        format!("https://mainnet.infura.io/v3/{infura_api}")
    ).unwrap();

    let null_add: Address = "0x0000000000000000000000000000000000000000".parse().unwrap();
    let poloniex_add: Address = "0x32Be343B94f860124dC4fEe278FDCBD38C102D88".parse().unwrap();


    // let provider = get_provider("Infura", &infura_api).await.unwrap();


    let last_block = provider.get_block_number().await.unwrap();

    for i in ((last_block.as_u64()-20)..last_block.as_u64()) {
        let block_no = ethers_core::types::BlockId::from(i);
        println!("{:?}", block_no);
        let null_bal = provider.get_balance(null_add, None).await;
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

    let client: Provider<Ws> =
        Provider::<Ws>::connect(
                format!("wss://mainnet.infura.io/ws/v3/{infura_api}")
            )
            .await
            .unwrap();
    let client = Arc::new(client);

    let erc20_transfer_filter =
        Filter::new().from_block(last_block - 10000).event("Transfer(address,address,uint256)");

    let mut stream = client.get_logs_paginated(&erc20_transfer_filter, 10).take(5);

    while let Some(res) = stream.next().await {
        let log = res.unwrap();
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            log.topics.get(1),
            log.topics.get(2),
            U256::decode(log.data)
        );
    }


}