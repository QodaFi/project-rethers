#![allow(unused)]

use async_trait::*;
use rethers::*;
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() {
  let address = str_to_H160("0xD8899C206FDD258057059C0AF431E8465CC48B14"); // qUSDCMAR23
  let topic = "FixedRateLoan(uint8,address,address,uint256,uint256,uint256,uint64)";
  let provider_url = "wss://moonriver.blastapi.io/e8db499b-1356-4017-9fc9-e4c9df38dd93";
  let header = "quoteSide,borrower,lender,amountPV,amountFV,feeIncurred,APR";

  let start_block = 3480770;
  let end_block = 3580770;
  
  run_event_query_with_header(
    provider_url,
    vec![address],
    topic,
    start_block,
    end_block,
    500,
    header
  ).await;
    
}

