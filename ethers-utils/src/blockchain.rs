use async_trait::*;
use ethers::prelude::*;
use crate::*;

#[async_trait]
pub trait MempoolListener {

  async fn on_start(&mut self, provider: &Provider<Ws>);

  async fn on_msg(&mut self, provider: &Provider<Ws>, tx: Transaction);  
  
  async fn listen(&mut self, provider_url: &str) {

    let provider = get_ws_provider(provider_url).await;

    self.on_start(&provider).await;

    let mut stream = provider.subscribe_pending_txs().await.unwrap();

    println!("[{}] Listening to mempool...", fmt_timestamp(current_time()));
    while let Some(tx_hash) = stream.next().await {
      let maybe_tx = provider.get_transaction(tx_hash).await.unwrap_or_else(|_| None);
      if let Some(tx) = maybe_tx {
        self.on_msg(&provider, tx).await;
      }
    }
  }
  
}

#[async_trait]
pub trait LogFetcher {

  async fn on_fetched(&mut self, provider: &Provider<Ws>, logs: Vec<Log>);
  
  async fn fetch_logs(
    &mut self,
    provider: &Provider<Ws>,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    prior_blocks: u64,
    chunk_size: u64
  ) {

    let latest_block = get_latest_block(&provider).await;
    
    let logs = get_logs_by_chunk(
      &provider,
      addresses,
      topics,
      latest_block - prior_blocks,
      latest_block,
      chunk_size
    ).await;
    
    self.on_fetched(&provider, logs).await;     
  }

  
  async fn fetch_logs_init_provider(
    &mut self,
    provider_url: &str,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    prior_blocks: u64,
    chunk_size: u64
  ) {
    
    let provider = get_ws_provider(provider_url).await;
    self.fetch_logs(
      &provider,
      addresses,
      topics,
      prior_blocks,
      chunk_size
    ).await;
  }

  async fn fetch_logs_historical(
    &mut self,
    provider: &Provider<Ws>,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    start_block: u64,
    end_block: u64,
    chunk_size: u64
  ) {

    let logs = get_logs_by_chunk(
      &provider,
      addresses,
      topics,
      start_block,
      end_block,
      chunk_size
    ).await;

    self.on_fetched(&provider, logs).await; 
  }

  async fn fetch_logs_historical_init_provider(
    &mut self,
    provider_url: &str,
    addresses: Vec<H160>,
    topics: Vec<H256>,
    start_block: u64,
    end_block: u64,
    chunk_size: u64
  ) {

    let provider = get_ws_provider(provider_url).await;

    self.fetch_logs_historical(
      &provider,
      addresses,
      topics,
      start_block,
      end_block,
      chunk_size
    ).await;
  }
  
}
