pub mod connect;
pub mod cli;


use subxt::{OnlineClient, PolkadotConfig};
use subxt::ext::sp_core::sr25519::Pair;
use subxt::tx::PairSigner;
use subxt::utils::{ H256};
use hex_literal::hex;

use sp_keyring::AccountKeyring;
use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use std::collections::HashMap;
use cli::Args;
use connect::{connect_to_rpc,connect_to_ws};
// ------------------




#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    // Main App
    match args {
        // 
        Args {current_era} =>{
            todo!();
        },
        Args {current_era, after } => {
            todo!();
        },
        Args {adjust_max} =>{
            todo!();
        },
        Args {current_era,after, adjust_max} =>{
            todo!();
        },
        Args {current_era,after, adjust_max, adjust_total_stake} =>{
            todo!();
        },
        Args {after, run_n_txn} =>{
            todo!();
        },
        Args {current_era,adjust_max,run_n_txn} =>{
            todo!();
        },
        Args {current_era, after, adjust_max,run_n_txn} =>{
            todo!()
        },
        Args {current_era, after, adjust_max, adjust_total_stake, run_n_txn} =>{
            todo!()
        },
        _ => {Println!("Input the correct Combinations")}

    }
            
    Ok(())
}

