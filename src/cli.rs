
pub use cli::*;

pub mod cli {
    use clap::{Arg, Parser};
    use std::collections::HashMap;
    use tokio::net::TcpStream; 
    use tokio_tungstenite::{tungstenite::Message};
    use serde::{Serialize, Deserialize};
    
     // Setting up clap
  #[derive(Parser, Debug)]
  #[command(author, version, about, long_about = None)]
   pub struct Args {
      /// Getting this Era Inflation
      #[arg(short, long, default_value_t = true)]
      current_era: bool,
  
      /// Get the Inflation after N Era have passed
      #[arg(short, long)]
      after: u8,

      /// Adjusting Max inflation to N
      #[arg(short, long)]
      adjust_max: u8,

      /// Adjusting total staked to N
      #[arg(short, long)]
      adjust_total_stake: u128,

      /// Dry run given txn in N times and return the user fees and inflation
      #[arg(short, long)]
      run_n_txn: (Vec<u8>,u8),

    // Upcoming implemnation is to tweak registered parachains with some custom Configs      
    //----------------------------------------

    let mut params:HashMap<&str,u8> = HashMap::new();

    #[derive(Serialize,Deserialize)]
    pub struct WMessage {
        jsonrpc: String,
        id: u8,
        method: String,
        params:Vec<HashMap<String,u8>>
    }
}

}
 