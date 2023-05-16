use frame_support::{Blake2_256, StorageHasher};
use subxt::{OnlineClient, PolkadotConfig};
use subxt::ext::sp_core::sr25519::Pair;
use subxt::tx::PairSigner;
use subxt::utils::{ H256};
use hex_literal::hex;
use tokio::net::TcpStream; 
use tokio_tungstenite::{connect_async, tungstenite::Message};
use sp_keyring::AccountKeyring;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
// ------------------

#[subxt::subxt(runtime_metadata_path = "testmetadata.scale")]
pub mod runtime {}


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {

    // Ws  Client
    let (mut ws_stream,response) = connect_async("ws://127.0.0.1:8000").await.unwrap();
    
    let mut params:HashMap<&str,u8> = HashMap::new();

    #[derive(Serialize,Deserialize)]
    pub struct WMessage<'a> {
        jsonrpc: &'a str,
        id: u8,
        method: &'a str,
        params:Vec<HashMap<&'a str,u8>>
    }



    // ---------
    let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8000").await.unwrap();
    // Sending a txn testing
    let signer = PairSigner::<PolkadotConfig,Pair>::new(AccountKeyring::Alice.pair());
    let alice = AccountKeyring::Alice.to_account_id();
    let dest = AccountKeyring::Bob.to_account_id();


    params.insert("count",10);

    let message = WMessage {
        jsonrpc: "2.0",
        id: 1,
        method: "dev_newBlock",
        params: vec![params]
    };

    // testing sending a message
   // let message =  Message::text(serde_json::to_string(&message));
    // ws_stream.send(message).await.expect("Failed to send");
    // // Reading the back stream
    // while let Some(Ok(message)) = ws_stream.next().await{
    //     match message {
    //         Message::Text(text) => {
    //             println!("{:?}",text)
    //         },
    //         _=> println!("Some other message")
    //     }
    // }

    // Pallet Balances
    // get total issuance
    let total_issuance_type = runtime::storage().balances().total_issuance();
    let total_issuance = api.storage().at_latest().await?.fetch(&total_issuance_type).await?.unwrap();
    
    println!("Total Issuance: {}",total_issuance);
    
    // Pallet Staking
    //1. Get Era duration in blocks
    //2. Get the current Era
    //3. Total staked per Era
    //4. Total payout per Era
    let era_duration :u32= 3600;
    let current_era_type = runtime::storage().staking().current_era();
    let current_era = api.storage().at_latest().await?.fetch(&current_era_type).await?.unwrap();

    println!("Current era : {}",current_era);

    // Era Total Stake
    let era_total_stake_type = runtime::storage().staking().eras_total_stake(current_era);
    let current_era_staked = api.storage().at_latest().await?.fetch(&era_total_stake_type).await?.unwrap();
    

    // Pallet Session



    // Pallet Treasury

    

    Ok(())
}

