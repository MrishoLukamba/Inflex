

#[subxt::subxt(runtime_metadata_path = "testmetadata.scale")]
pub mod runtime {}

pub mod query {

    // Pallet Balances
    // get total issuance
    pub async fn get_total_issuance() ->Result<u128,Box<dyn std::error::Error>>{
        let total_issuance_type = runtime::storage().balances().total_issuance();
        let total_issuance = api.storage().at_latest().await?.fetch(&total_issuance_type).await?.unwrap();
    
        println!("Total Issuance: {}",total_issuance);
        Ok(total_issuance)
    }
    
    // Pallet Staking
    //1. Get Era duration in blocks
    //2. Get the current Era
    //3. Total staked per Era
    //4. Total payout per Era
    pub async fn get_stake_info() -> Result<(u128,u128,u32),Box<dyn std::error::Error>>{

        let era_duration :u32= 3600;
        let current_era_type = runtime::storage().staking().current_era();
        let current_era = api.storage().at_latest().await?.fetch(&current_era_type).await?.unwrap();

        println!("Current era : {}",current_era);

        // Era Total Stake
        let era_total_stake_type = runtime::storage().staking().eras_total_stake(current_era);
        let current_era_staked = api.storage().at_latest().await?.fetch(&era_total_stake_type).await?.unwrap();
    
        Ok(1000000, current_era_staked,current_era) // First value for Demo
        //Return is (3,4,2)
    }
    
    // Pallet Session


    // Pallet Treasury
}
    