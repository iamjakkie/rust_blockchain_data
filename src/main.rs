// TODO: Initialize the RPC client through Infura
// libs: ethers.rs




use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::fmt::LowerHex;

use ethers::prelude::*;
// use ethers_core::types::Address;
// use ethers_providers::{Provider, Http, Middleware};
use std::convert::TryFrom;



#[derive(Serialize, Deserialize, Debug)]
struct postParams {
    id: String,
    jsonrpc: String,
    method: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct blockPostParams {
    id: String,
    jsonrpc: String,
    method: String,
    params: (String, bool),
}

#[derive(Serialize, Deserialize, Debug)]
struct blockResponse {

}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = reqwest::Client::new();
//     let params = postParams {
//         id: "1".into(),
//         jsonrpc: "2.0".into(),
//         method: "eth_blockNumber".into(),
//     };
//     let resp = client.post("https://eth-mainnet.g.alchemy.com/v2/api_key")
//         .json(&params)
//         .send()
//         .await?
//         .text()
//         .await?;
//     let resp_formatted:Value = serde_json::from_str(&resp)?;
//     let res = resp_formatted["result"].as_str().unwrap();
//     let last_block = u64::from_str_radix(res.trim_start_matches("0x"), 16).unwrap();
//     println!("Last block: {}", last_block);

    

//     for i in last_block-10..last_block {
//         let hex_block = format!("{i:#x}");
//         // println!("{}", hex_block);

//         let blockParams = blockPostParams {
//             id: "1".into(),
//             jsonrpc: "2.0".into(),
//             method: "eth_getBlockByNumber".into(),
//             params: (hex_block.as_str().into(), false)
//         };
//         // println!("{:?}", blockParams);
//         let resp = client.post("https://eth-mainnet.g.alchemy.com/v2/api_key")
//             .json(&blockParams)
//             .send()
//             .await?
//             .text()
//             .await?;

//         // println!("{:?}", resp);
    
//         let resp_formatted:Value = serde_json::from_str(&resp)?;
//         let resp_result = serde_json::from_value::<Value>(resp_formatted["result"].clone()).unwrap();
    
//         let res = resp_result["gasUsed"].as_str().unwrap();
//         let gasUsed = u64::from_str_radix(res.trim_start_matches("0x"), 16).unwrap();
    
//         println!("Block_no: {}, gas used: {}", i, gasUsed);
//     }

    
//     Ok(())
// }


// pub fn main(){
//     let provider = Provider::<Http>::try_from(
//         "https://mainnet.infura.io/v3/"
//     )?;
    
//     let block = provider.get_block(100u64).await?;
//     println!("Got block: {}", serde_json::to_string(&block)?);
    
//     let addr = "0x89d24a6b4ccb1b6faa2625fe562bdd9a23260359".parse::<Address>()?;
//     let code = provider.get_code(addr, None).await?;
//     println!("Got code: {}", serde_json::to_string(&code)?);

// }

// ETH JSON-RPC Spec: https://ethereum.github.io/execution-apis/api-documentation/

#[tokio::main]
async fn main() {
    let provider_res = Provider::<Http>::try_from(
        "https://mainnet.infura.io/v3/"
    );
    // Write error handling
    // match provider_res {
    //     Ok(T) -> {},
    //     _ -> {}
    // }

    if let Err(e) = provider_res {
        println!("Oh no! {:?}", provider_res);
    }

    let provider = provider_res.unwrap();

    let block_res = provider.get_block(100u64).await;

    provider.get_balance(from, block)
    // Write error handling

    let block = block_res.unwrap();

    println!("Got block: {}", serde_json::to_string(&block).unwrap());

    // eth balance for

    // TODO: get eth balance for a wallet at a past (but recent) block, say, 10 blocks ago from now.

    // TODO: get historical emitted events from a smart contract - eg. listen on Transfer() events for ETH.

    // TODO: figure out which events are emitted from a swap on DEX.
    // Assuming OHLCV data constructed only from completed swaps, what is happening E2E during a swap on a DEX? What smart contract functions are called, what events are being emitted, are there any other contracts at play here?

    // TODO: query smart contract for relevant data ^ for historical blocks.

    
    // TODO: find if there is a way to identify relevant smart contracts for a given DEX (i.e. Uniswap)
    
}

    /**

    Calldata:

    0x123125012471291029312039120938109230123812093102312983
    0x12312501 - function signature
    the rest - parameters

    ABI specificies how to interface with smart contract.
        - copy past from etherscan
        - generate your copy of the ABI from your local copy of the contract code

    Uniswap Swap:

    0x3593564c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000063eaa36700000000000000000000000000000000000000000000000000000000000000020b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000ddd7dc5631c6670000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000ddd7dc5631c667000000000000000000000000000000000000000000000000000000000583bd4000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002bc02aaa39b223fe8d0a0e5c4f27ead9083c756cc20001f4a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48000000000000000000000000000000000000000000

    */