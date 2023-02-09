use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::fmt::LowerHex;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let params = postParams {
        id: "1".into(),
        jsonrpc: "2.0".into(),
        method: "eth_blockNumber".into(),
    };
    let resp = client.post("https://eth-mainnet.g.alchemy.com/v2/api_key")
        .json(&params)
        .send()
        .await?
        .text()
        .await?;
    let resp_formatted:Value = serde_json::from_str(&resp)?;
    let res = resp_formatted["result"].as_str().unwrap();
    let last_block = u64::from_str_radix(res.trim_start_matches("0x"), 16).unwrap();
    println!("Last block: {}", last_block);

    

    for i in (last_block-10..last_block) {
        let hex_block = format!("{:x}", i);

        let blockParams = blockPostParams {
            id: "1".into(),
            jsonrpc: "2.0".into(),
            method: "eth_getBlockByNumber".into(),
            params: (hex_block.into(), false)
        };
        println!("{:?}", blockParams);
        let resp = client.post("https://eth-mainnet.g.alchemy.com/v2/api_key")
            .json(&blockParams)
            .send()
            .await?
            .text()
            .await?;
    
        let resp_formatted:Value = serde_json::from_str(&resp)?;
        let resp_result = serde_json::from_value::<Value>(resp_formatted["result"].clone()).unwrap();
    
        let res = resp_result["gasUsed"].as_str().unwrap();
    
        println!("{:?}", res);
    }

    
    Ok(())
}
