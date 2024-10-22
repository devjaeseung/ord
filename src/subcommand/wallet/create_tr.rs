use super::*;

#[derive(Deserialize, Serialize)]
pub struct Output {
    pub addresses: Vec<Address<NetworkUnchecked>>,
}

#[derive(Debug, Parser)]
pub(crate) struct CreateTR {
    #[arg(short, long, help = "Create TapRootAddress <NUMBER>.")]
    number: Option<u64>,
}

impl CreateTR {
    pub(crate) fn run(self, wallet: Wallet) -> SubcommandResult {
        let mut addresses: Vec<Address<NetworkUnchecked>> = Vec::new();
        println!("[createTR.rs] Create TapRoot Address");
        
        // let number_of_addresses = self.number.unwrap_or(1);
        // println!("[createTR.rs] Number of addresses to generate: {}", number_of_addresses);
        // 
        // for i in 0..number_of_addresses {
        //     println!("[receive.rs] Generating address {} of {}", i + 1, number_of_addresses);
        //     let new_address = wallet
        //         .bitcoin_client()
        //         .get_new_address(None, Some(bitcoincore_rpc::json::AddressType::Bech32m))?;
        //     println!("[receive.rs] Generated address: {:?}", new_address);
        //     addresses.push(new_address);
        // }
        // 
        // println!("[receive.rs] All generated addresses: {:?}", addresses);
        
        Ok(Some(Box::new(Output { addresses })))
    }
}



