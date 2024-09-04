use getopts::Options;
use std::env;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process;

use crate::api;
use crate::api::{api, blocking};

/// GET /api/v1/historical-price?currency=EUR&timestamp=1500000000
/// pub fn historical_price(currency: &str, timestamp: &str)
pub fn historical_price(currency: &str, timestamp: &str) {
    let _res = api::blocking(&format!(
        "v1/historical-price?currency={}&timestamp={}",
        &(&currency).to_string(),
        &(&timestamp).to_string()
    ));
}
/// GET /api/block/:hash/txid/:index
/// <https://mempool.space/docs/api/rest#get-block-transaction-id>
pub fn block_txid(block_hash: &str, txindex: &str) {
    let _res = api::blocking(&format!("block/{}/txid/{}", block_hash, txindex));
}
/// GET /api/block/:hash/txids
/// <https://mempool.space/docs/api/rest#get-block-transaction-ids>
pub fn block_txids(block_hash: &str) {
    let _res = api::blocking(&format!("block/{}/txids", block_hash));
}
/// GET /api/block/:hash/txs[/:start_index] (start_index % 25 = 0)
/// <https://mempool.space/docs/api/rest#get-block-transactions>
pub fn block_txs(block_hash: &str, start_index: &str) {
    let start_index_int = start_index.parse::<i32>().unwrap_or(0);
    if start_index_int % 25 == 0 {
        let _res = api::blocking(&format!("block/{}/txs/{}", block_hash, start_index));
    } else {
        let _res = api::blocking(&format!("block/{}/txs/{}", block_hash, &"0"));
    }
}
/// GET /api/v1/blocks[/:startHeight]
/// <https://mempool.space/docs/api/rest#get-blocks>
pub fn blocks(start_height: &str, print: bool) {
    let blocks_tip_height = api::api("blocks_tip_height", "extraneous_arg", print);
    let blocks_tip_height_int = blocks_tip_height.parse::<i32>().unwrap_or(0);
    let start_height_int = start_height.parse::<i32>().unwrap_or(0);
    if start_height_int >= 0 && start_height_int <= blocks_tip_height_int {
        let _res = blocking(&format!("v1/blocks/{}", start_height));
    } else {
        let _res = blocking(&"v1/blocks".to_string());
    }
}
/// GET /api/v1/blocks-bulk/:minHeight[/:maxHeight]
/// <https://mempool.space/docs/api/rest#get-blocks-bulk>
pub fn blocks_bulk(min_height: &str, max_height: &str, print: bool) {
    let min_height_int = min_height.parse::<i32>().unwrap_or(0);
    let max_height_int = max_height.parse::<i32>().unwrap_or(0);
    if min_height_int >= 0 && max_height_int >= 0 && min_height_int < max_height_int {
        let _res = blocking(&format!("v1/blocks-bulk/{}/{}", min_height, max_height));
    } else if min_height_int >= 0 && max_height_int >= 0 && min_height_int >= max_height_int {
        let _res = blocking(&format!("v1/blocks-bulk/{}/{}", max_height, min_height));
    } else {
        let blocks_tip_height = api::api("blocks_tip_height", "extraneous_arg", print);
        let _res = blocking(&format!("v1/blocks-bulk/{}/{}", min_height, blocks_tip_height));
    }
    print!("This API is disabled. Set config.MEMPOOL.MAX_BLOCKS_BULK_QUERY to a positive number to enable it.");
}
/// GET /api/v1/mining/pool/:slug/blocks/\[:blockHeight]
/// <https://mempool.space/docs/api/rest#get-mining-pool-blocks>
pub fn mining_pool_blocks(slug: &str, blockheight: &str) {
    let _res = blocking(&format!(
        "v1/mining/pool/{}/blocks/{}",
        &(&slug).to_string(),
        &(&blockheight).to_string()
    ));
}

/// USAGE
///
/// ``mempool-space --difficulty_adjustment (flagged)``
///
/// ``mempool-space_difficulty_adjustment (executable)``
///
/// - Flags follow the [mempool.space api/rest](https://mempool.space/docs/api/rest) (replace dashes with underscores)
///
/// - Flags invoke the installed executable
///
/// DASHBOARD TUI INTERFACE (WIP)
///
/// ``mempool-space --dashboard (flagged)``
///
/// ``mempool-space_dashboard (executable)``
///
///
/// <https://mempool.space/docs/api/rest>
/// - [API/REST](https://mempool.space/docs/api/rest)
///     - [GENERAL](https://mempool.space/docs/api/rest#get-difficulty-adjustment)
///         - GET /api/v1/difficulty-adjustment
///             - <https://mempool.space/api/v1/difficulty-adjustment>
///         - GET /api/v1/prices
///             - <https://mempool.space/api/v1/prices>
///         - GET /api/v1/historical-price?currency=EUR&timestamp=1500000000
///             - <https://mempool.space/api/v1/historical-price?currency=EUR&timestamp=1500000000>
///     - [ADDRESSES](https://mempool.space/docs/api/rest#get-address)
///         - GET /api/address/:address
///             - <https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv>
///         - GET /api/address/:address/txs
///             - <https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs>
///         - GET /api/address/:address/txs/chain
///             - <https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/chain>
///         - GET /api/address/:address/txs/mempool
///             - <https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/mempool>
///         - GET /api/address/:address/utxo
///             - <https://mempool.space/api/address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY/utxo>
///         - GET /api/v1/validate-address/:address
///             - <https://mempool.space/api/v1/validate-address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY>
///     - [BLOCKS](https://mempool.space/docs/api/rest#get-block)
///         - GET /api/block/:hash
///             - <https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce>
///         - GET /api/block/:hash/header
///             - <https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/header>
///         - GET /api/block-height/:height
///             - <https://mempool.space/api/block-height/615615>
///         - GET /api/v1/mining/blocks/timestamp/:timestamp
///             - <https://mempool.space/api/v1/mining/blocks/timestamp/1672531200>
///         - GET /api/block/:hash/raw
///             - <https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/raw>
///         - GET /api/block/:hash/status
///             - <https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/status>
///         - GET /api/blocks/tip/height
///             - <https://mempool.space/api/blocks/tip/height>
///         - GET /api/blocks/tip/hash
///             - <https://mempool.space/api/blocks/tip/hash>
///         - GET /api/block/:hash/txid/:index
///             - <https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txid/218>
///         - GET /api/block/:hash/txids
///             - <https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txids>
///         - GET /api/block/:hash/txs[/:start_index]
///             - <https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs>
///         - GET /api/v1/blocks[/:startHeight]
///             - <https://mempool.space/api/v1/blocks/730000>
///         - GET /api/v1/blocks-bulk/:minHeight[/:maxHeight]
///             - <https://mempool.space/api/v1/blocks-bulk/100000/100000> (Enterprise)
///     - [MINING](https://mempool.space/docs/api/rest#get-mining-pools)
///         - GET /api/v1/mining/pools[/:timePeriod]
///             - <https://mempool.space/api/v1/mining/pools/1w>
///         - GET /api/v1/mining/pool/:slug
///             - <https://mempool.space/api/v1/mining/pool/antpool>
///         - GET /api/v1/mining/hashrate/pools/[\:timePeriod]
///             - <https://mempool.space/api/v1/mining/hashrate/pools/1m>
///         - GET /api/v1/mining/pool/:slug/hashrate
///             - <https://mempool.space/api/v1/mining/pool/foundryusa/hashrate>
///         - GET /api/v1/mining/pool/:slug/blocks/[\:blockHeight]
///             - <https://mempool.space/api/v1/mining/pool/luxor/blocks/730000>
///         - GET /api/v1/mining/hashrate/[\:timePeriod]
///             - <https://mempool.space/api/v1/mining/hashrate/3d>
///         - GET /api/v1/mining/pool/:slug/blocks/[\:blockHeight]
///             - <https://mempool.space/api/v1/mining/pool/luxor/blocks/730000>
///         - GET /api/v1/mining/difficulty-adjustments/[\:interval]
///             - <https://mempool.space/api/v1/mining/difficulty-adjustments/1m>
///         - GET /api/v1/mining/reward-stats/:blockCount
///             - <https://mempool.space/api/v1/mining/reward-stats/100>
///         - GET /api/v1/mining/blocks/fees/:timePeriod
///             - <https://mempool.space/api/v1/mining/blocks/fees/1w>
///         - GET /api/v1/mining/blocks/rewards/:timePeriod
///             - <https://mempool.space/docs/api/rest#get-block-rewards>
///         - GET /api/v1/mining/blocks/fee-rates/:timePeriod
///             - <https://mempool.space/api/v1/mining/blocks/fee-rates/1m>
///         - GET /api/v1/mining/blocks/sizes-weights/:timePeriod
///             - <https://mempool.space/api/v1/mining/blocks/sizes-weights/3y>
///         - GET /api/v1/mining/blocks/predictions/:timePeriod
///             - <https://mempool.space/api/v1/mining/blocks/predictions/3y>
///         - GET /api/v1/mining/blocks/audit/score/:blockHash
///             - <https://mempool.space/api/v1/mining/blocks/audit/score/000000000000000000032535698c5b0c48283b792cf86c1c6e36ff84464de785>
///         - GET /api/v1/mining/blocks/audit/scores/:startHeight
///             - <https://mempool.space/api/v1/mining/blocks/audit/scores/820000>
///         - GET /api/v1/block/:blockHash/audit-summary
///             - <https://mempool.space/api/v1/block/00000000000000000000f218ceda7a5d9c289040b9c3f05ef9f7c2f4930e0123/audit-summary>
///     - [FEES](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)
///         - GET /api/v1/fees/mempool-blocks
///             - <https://mempool.space/api/v1/fees/mempool-blocks>
///         - GET /api/v1/fees/recommended
///             - <https://mempool.space/api/v1/fees/recommended>
///     - [MEMPOOL](https://mempool.space/docs/api/rest#get-mempool)
///         - GET /api/mempool
///             - <https://mempool.space/api/mempool>
///         - GET /api/mempool/txids
///             - <https://mempool.space/api/mempool/txids>
///         - GET GET /api/mempool/recent
///             - <https://mempool.space/api/mempool/recent>
///         - GET /api/v1/replacements
///             - <https://mempool.space/api/v1/replacements>
///         - GET /api/v1/fullrbf/replacements
///             - <https://mempool.space/api/v1/fullrbf/replacements>
///     - [TRANSACTIONS](https://mempool.space/docs/api/rest#get-cpfp)
///         - GET /api/v1/cpfp
///             - <https://mempool.space/api/v1/cpfp/be6ba3c97d65478534333dc9a6db159b538c3722bd18bea43765e95c7139a8e6>
///         - GET /api/tx/:txid
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521>
///         - GET /api/tx/:txid/hex
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521/hex>
///         - GET /api/tx/:txid/merkleblock-proof
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521/merkleblock-proof>
///         - GET /api/tx/:txid/merkle-proof
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521/merkle-proof>
///         - GET /api/tx/:txid/outspend/:vout
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521/outspend/3>
///         - GET /api/tx/:txid/outspends
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521/outspends>
///         - GET /api/tx/:txid/raw
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521/raw>
///         - GET /apiv1/tx/:txId/rbf
///             - <https://mempool.space/api/v1/tx/2e95ff9094df9f3650e3f2abc189250760162be89a88f9f2f23301c7cb14b8b4/rbf>
///         - GET /api/tx/:txid/status
///             - <https://mempool.space/api/tx/15e10745f15593a899cef391191bdd3d7c12412cc4696b7bcb669d0feadc8521/status>
///         - GET /api/v1/transaction-times
///             - <https://mempool.space/api/v1/transaction-times?txId[]=51545ef0ec7f09196e60693b59369a134870985c8a90e5d42655b191de06285e&txId[]=6086089bd1c56a9c42a39d470cdfa7c12d4b52bf209608b390dfc4943f2d3851>
///         - POST /api/tx
///             - \<TODO\>
///     - [LIGHTNING](https://mempool.space/docs/api/rest#get-lightning-network-stats)
///         - GET /api/v1/lightning/statistics/:interval
///             - <https://mempool.space/api/v1/lightning/statistics/latest>
///         - GET /api/v1/lightning/search?searchText=:query
///             - <https://mempool.space/api/v1/lightning/search?searchText=ACINQ>
///         - GET /api/v1/lightning/nodes/country/:country
///             - <https://mempool.space/api/v1/lightning/nodes/country/ch>
///         - GET /api/v1/lightning/nodes/countries
///             - <https://mempool.space/api/v1/lightning/nodes/countries>
///         - GET /api/v1/lightning/nodes/isp/:isp
///             - <https://mempool.space/api/v1/lightning/nodes/isp/16509>
///         - GET /api/v1/lightning/nodes/isp-ranking
///             - <https://mempool.space/api/v1/lightning/nodes/isp-ranking>
///         - GET /api/v1/lightning/nodes/rankings
///             - <https://mempool.space/api/v1/lightning/nodes/rankings>
///         - GET /api/v1/lightning/nodes/rankings/liquidity
///             - <https://mempool.space/api/v1/lightning/nodes/rankings/liquidity>
///         - GET /api/v1/lightning/nodes/rankings/connectivity
///             - <https://mempool.space/api/v1/lightning/nodes/rankings/connectivity>
///         - GET /api/v1/lightning/nodes/rankings/age
///             - <https://mempool.space/api/v1/lightning/nodes/rankings/age>
///         - GET /api/v1/lightning/nodes/:pubKey
///             - <https://mempool.space/api/v1/lightning/nodes/033ac2f9f7ff643c235cc247c521663924aff73b26b38118a6c6821460afcde1b3>
///         - GET /api/v1/lightning/nodes/:pubKey/statistics
///             - <https://mempool.space/api/v1/lightning/nodes/033ac2f9f7ff643c235cc247c521663924aff73b26b38118a6c6821460afcde1b3/statistics>
///         - GET /api/v1/lightning/channels/:channelId
///             - <https://mempool.space/api/v1/lightning/channels/768457472831193088>
///         - GET /api/v1/lightning/channels/txids?txId[]=:txid
///             - <https://mempool.space/api/v1/lightning/channels/txids?txId[]=c3173549f502ede6440d5c48ea74af5607d88484c7a912bbef73d430049f8af4&txId[]=d78f0b41a263af3df91fa4171cc2f60c40196aaf8f4bde5d1c8ff4474cfe753b>
///         - GET /api/v1/lightning/channels?public_key=:pubKey&status=:channelStatus
///             - <https://mempool.space/api/v1/lightning/channels?public_key=026165850492521f4ac8abd9bd8088123446d126f648ca35e60f88177dc149ceb2&status=open>
///         - GET /api/v1/lightning/channels-geo
///             - <https://mempool.space/api/v1/lightning/channels-geo>
///         - GET /api/v1/lightning/channels-geo/:pubKey
///             - <https://mempool.space/api/v1/lightning/channels-geo/03d607f3e69fd032524a867b288216bfab263b6eaee4e07783799a6fe69bb84fac>
///     - [ACCELERATOR (Public)](https://mempool.space/docs/api/rest#accelerator-estimate)
///         - POST /v1/services/accelerator/estimate
///             - `curl -H "X-Mempool-Auth: stacksats" -X POST -sSLd "txInput=ee13ebb99632377c15c94980357f674d285ac413452050031ea6dcd3e9b2dc29" "https://mempool.space/api/v1/services/accelerator/estimate"`
///         POST /v1/services/payments/bitcoin
///             - `curl -X POST -sSLd "product=ee13ebb99632377c15c94980357f674d285ac413452050031ea6dcd3e9b2dc29&amount=12500" "https://mempool.space/api/v1/services/payments/bitcoin"`
///         - GET /api/v1/services/accelerator/accelerations
///             - <https://mempool.space/api/v1/services/accelerator/accelerations>
///         - GET /api/v1/services/accelerator/accelerations/history
///             - <https://mempool.space/api/v1/services/accelerator/accelerations/history?blockHash=00000000000000000000482f0746d62141694b9210a813b97eb8445780a32003>
///             - `curl https://raw.githubusercontent.com/mempool/mining-pools/master/pools-v2.json`
///     - [ACCELERATOR (Authenticated) TODO](https://mempool.space/docs/api/rest#accelerator-top-up-history)
///         - GET /api/v1/services/accelerator/top-up-history
///             - <https://mempool.space/api/v1/services/accelerator/top-up-history>
///         - GET /api/v1/services/accelerator/balance
///             - <https://mempool.space/api/v1/services/accelerator/balance>
///         - POST /v1/services/accelerator/accelerate
///             - `curl -H "X-Mempool-Auth: stacksats" -X POST -sSLd "txInput=ee13ebb99632377c15c94980357f674d285ac413452050031ea6dcd3e9b2dc29&userBid=21000000" "https://mempool.space/api/v1/services/accelerator/accelerate"`
///         - GET /api/v1/services/accelerator/history?status=:status&details=:details
///             - <https://mempool.space/api/v1/services/accelerator/history?status=all&details=true>
///
#[derive(Debug, Default)]
pub struct Args {
    /// `mempool-space --version`
    pub version: Option<String>,
    /// `mempool-space --dashboard`
    /// invoke the tui (WIP)
    pub dashboard: Option<String>,
    /// `https://mempool.space/api/v1/difficulty-adjustment`
    pub difficulty_adjustment: Option<String>,
    /// `https://mempool.space/api/v1/prices`
    pub prices: Option<String>,
    /// `https://mempool.space/api/v1/historical-price`
    pub historical_price: Option<String>,
    /// `https://mempool.space/api/v1/historical-price?currency=USD`
    pub currency: Option<String>,
    /// `https://mempool.space/api/v1/historical-price?currency=USD?timestamp=0`
    pub timestamp: Option<String>,

    /// `https://mempool.space/api/address/<ADDRESS>`
    pub address: Option<String>,
    /// `https://mempool.space/api/address/<ADDRESS>/txs`
    pub address_txs: Option<String>,
    /// `https://mempool.space/api/address/<ADDRESS>/txs/chain`
    pub address_txs_chain: Option<String>,
    /// `https://mempool.space/address/<ADDRESS>/txs/mempool`
    pub address_txs_mempool: Option<String>,
    /// `https://mempool.space/api/address/<ADDRESS>/utxo`
    pub address_utxo: Option<String>,
    /// `https://mempool.space/api/validate-address/<ADDRESS>`
    pub validate_address: Option<String>,

    /// `https://mempool.space/api/block/<BLOCK_HASH>`
    pub block: Option<String>,
    /// `https://mempool.space/api/block/<BLOCK_HASH>/header`
    pub block_header: Option<String>,
    /// `https://mempool.space/api/block-height/<BLOCK_HEIGHT>`
    pub block_height: Option<String>,
    /// `https://mempool.space/api/v1/mining/blocks/timestamp/<UTC_SECS>`
    pub blocks_timestamp: Option<String>,
    /// `https://mempool.space/api/block/<BLOCK_HASH>/raw`
    pub block_raw: Option<String>,
    /// `https://mempool.space/api/block/<BLOCK_HASH>/status`
    pub block_status: Option<String>,
    /// `https://mempool.space/api/blocks/tip/height`
    pub blocks_tip_height: Option<String>,
    /// `https://mempool.space/api/blocks/tip/hash`
    pub blocks_tip_hash: Option<String>,
    /// `https://mempool.space/api/block/:hash/txid/:index`
    pub block_txid: Option<String>, //HASH
    /// `https://mempool.space/api/block/:hash/txid/:index`
    pub block_txindex: Option<String>, //INDEX
    /// `https://mempool.space/api/block/<TXID>`
    pub block_txids: Option<String>,
    /// `https://mempool.space/api/block/<BLOCK_HASH>/txs`
    pub block_txs: Option<String>,
    /// `https://mempool.space/api/v1/blocks/<BLOCKS_START_HEIGHT>`
    pub start_index: Option<String>,
    /// `https://mempool.space/api/v1/blocks/<BLOCKS_START_HEIGHT>`
    pub blocks: Option<String>,
    /// `https://mempool.space/api/v1/blocks-bulk/<MIN_HEIGHT>/<MAX_HEIGHT>`
    pub blocks_bulk: Option<String>,
    /// `https://mempool.space/api/v1/blocks-bulk/<MIN_HEIGHT>/<MAX_HEIGHT>`
    pub min_height: Option<String>,
    /// `https://mempool.space/api/v1/blocks-bulk/<MIN_HEIGHT>/<MAX_HEIGHT>`
    pub max_height: Option<String>,

    /// `https://mempool.space/api/v1/mining/pools[/:timePeriod]`
    pub mining_pools: Option<String>,
    /// `https://mempool.space/api/v1/mining/pools[/:timePeriod]`
    pub timeperiod: Option<String>,

    /// `https://mempool.space/api/v1/mining/pool/:<SLUG>`
    pub mining_pool: Option<String>,
    /// slug
    pub slug: Option<String>,

    /// `https://mempool.space/api/v1/mining/difficulty-adjustments/[:interval]`
    pub difficulty_adjustments: Option<String>,
    /// interval
    pub interval: Option<String>,

    /// `https://mempool.space/api/v1/mining/blocks/audit/score/<BLOCKHASH>`
    pub blocks_audit_score: Option<String>,
    /// `https://mempool.space/api/v1/mining/blocks/audit/score/<BLOCKHASH>`
    pub blockhash: Option<String>,

    /// `https://mempool.space/api/v1/mining/blocks/audit/scores/<BLOCKHEIGHT>`
    pub blocks_audit_scores: Option<String>,
    /// `https://mempool.space/api/v1/mining/blocks/audit/scores/<BLOCKHEIGHT>`
    pub blockheight: Option<String>,

    /// `https://mempool.space/api/v1/mining/block/<BLOCKHASH>/audit-summary`
    pub block_audit_summary: Option<String>,
    // pub blockhash: Option<String>,
    //
    //
    //
    //
    //
    /// OPTOPT
    ///
    /// Configuration file.
    pub config: Option<PathBuf>,
    /// Server address.
    pub server: Option<String>,
    /// Authentication or delete token.
    pub auth: Option<String>,
    /// URL to shorten.
    pub url: Option<String>,
    /// Remote URL to download file.
    pub remote: Option<String>,
    /// Files to upload.
    pub files: Vec<String>,
    /// Whether if the file will disappear after being viewed once.
    pub oneshot: bool,
    /// Expiration time for the link.
    pub expire: Option<String>,
    /// Prettify the program output.
    pub prettify: bool,
    /// Whether if the server version should be printed.
    pub print_server_version: bool,
    /// List files on the server (file name, file size, expiry timestamp).
    pub list_files: bool,
    /// Delete files from server.
    pub delete: bool,
    /// Send filename header (give uploaded file a specific name).
    pub filename: Option<String>,
}

impl Args {
    /// Parses the command-line arguments.
    pub fn parse() -> Self {
        let mut opts = Options::new();

        //OPTFLAG
        opts.optflag("h", "help", "prints help information");
        opts.optflag("v", "version", "prints version information");
        opts.optflag("V", "server-version", "retrieves the server version");
        opts.optflag("l", "list", "lists files on the server");
        opts.optflag("d", "delete", "delete files from server");
        opts.optflag("o", "oneshot", "generates one shot links");
        opts.optflag("p", "pretty", "prettifies the output");

        // mempool-space_dashboard
        opts.optflag("", "dashboard", "invoke the tui (WIP)");

        // mempool api intercepts
        // VERSION
        // premeptive support v1,v2 etc...
        // opts.optopt("", "version", "api call version path (v1/...)", "VERSION");
        // GENERAL
        opts.optflag("", "difficulty_adjustment", "difficulty_adjustment api call");
        opts.optflag("", "prices", "prices api call");
        opts.optflag("", "historical_price", "historical_price api call");
        opts.optopt("", "timestamp", "timestamp api call", "TIMESTAMP");
        opts.optopt("", "currency", "currency api call", "CURRENCY");

        // ADDRESSES
        opts.optopt("", "address", "address api call", "ADDRESS");
        opts.optopt("", "address_txs", "address_txs api call", "ADDRESS_TXS");
        opts.optopt(
            "",
            "address_txs_chain",
            "address_txs_chain api call",
            "ADDRESS_TXS_CHAIN",
        );
        opts.optopt(
            "",
            "address_txs_mempool",
            "address_txs_mempool api call",
            "ADDRESS_TXS_MEMPOOL",
        );
        opts.optopt("", "address_utxo", "address_utxos api call", "ADDRESS_UTXO");
        opts.optopt("", "validate_address", "validate an address", "VALIDATE_ADDRESS");

        // BLOCK/S
        opts.optopt("", "block", "block api call", "BLOCK");
        opts.optopt("", "block_header", "block-header api call", "BLOCK_HEADER");

        // BLOCK_HEIGHT
        // REUSED
        opts.optopt("", "block_height", "block-height api call", "BLOCK_HEIGHT");
        //
        opts.optopt("", "blocks_timestamp", "blocks-timestamp api call", "BLOCKS_TIMESTAMP");
        opts.optopt("", "block_raw", "block-raw api call", "BLOCK_RAW");
        opts.optopt("", "block_status", "block-status api call", "BLOCK_STATUS");

        opts.optflag("", "blocks_tip_height", "GET /api/blocks/tip/height api call");
        opts.optflag("", "blocks_tip_hash", "GET /api/blocks/tip/hash api call");

        opts.optopt("", "block_txid", "block txid api call", "BLOCK_TXID");
        opts.optopt("", "block_txindex", "block_txindex api call", "BLOCK_TXINDEX");
        opts.optopt("", "block_txids", "block txids api call", "BLOCK_TXIDS");

        opts.optopt("", "block_txs", "block txs api call", "BLOCK_TXS");
        opts.optopt("", "start_index", "block txs api call", "START_INDEX");

        opts.optopt("", "blocks", "block txids api call", "BLOCKS_START_HEIGHT");

        opts.optflag("", "blocks_bulk", "block txids api call");
        opts.optopt("", "min_height", "block txids api call", "MIN_HEIGHT");
        opts.optopt("", "max_height", "block txids api call", "MAX_HEIGHT");

        // MINING
        opts.optflag("", "mining_pools", "mining_pools api call");
        opts.optopt("", "timeperiod", "mining_pools api call", "TIMEPERIOD");

        opts.optflag("", "mining_pool", "mining_pool api call");
        opts.optopt("", "slug", "mining_pool api call", "SLUG");

        opts.optflag("", "mining_hashrate_pools", "mining_hashrate_pools api call");
        opts.optopt("", "timeperiod", "mining_hashrate_pools api call", "TIMEPERIOD");

        opts.optflag("", "mining_pool_hashrate", "mining_pool_hashrate api call");
        opts.optopt("", "slug", "mining_pool_hashrate api call", "SLUG");

        opts.optflag("", "mining_pool_blocks", "mining_pool_hashrate api call");
        opts.optopt("", "slug", "mining_pool_hashrate api call", "SLUG");
        opts.optopt("", "blockheight", "mining_pool_hashrate api call", "BLOCKHEIGHT");

        opts.optflag("", "difficulty_adjustments", "difficulty_adjustments api call");
        opts.optopt("", "interval", "difficulty_adjustments api call", "INTERVAL");

        opts.optflag("", "blocks_audit_score", "blocks_audit_score api call");
        opts.optopt("", "block_hash", "blocks_audit_score api call", "BLOCK_HASH");

        opts.optflag("", "blocks_audit_scores", "blocks_audit_scores api call");
        //opts.optopt("", "blockheight", "blocks_audit_scores api call", "BLOCKHEIGHT");

        opts.optflag("", "block_audit_summary", "block_audit_summary api call");
        opts.optopt("", "blockhash", "block_audit_summary api call", "BLOCKHASH");

        // OPTOPT
        opts.optopt("c", "config", "sets the configuration file", "CONFIG");
        opts.optopt("s", "server", "sets the address of the rustypaste server", "SERVER");
        opts.optopt("a", "auth", "sets the authentication or delete token", "TOKEN");
        opts.optopt("u", "url", "sets the URL to shorten", "URL");
        opts.optopt("r", "remote", "sets the remote URL for uploading", "URL");
        opts.optopt("e", "expire", "sets the expiration time for the link", "TIME");
        opts.optopt("n", "filename", "sets and overrides the filename", "NAME");

        let env_args: Vec<String> = env::args().collect();
        let matches = match opts.parse(&env_args[1..]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Argument error: `{e}`");
                process::exit(1);
            }
        };

        //invoke the dashboard
        // DASHBOARD
        if matches.opt_present("dashboard") {
            api("dashboard", "", false);
        };
        //mempool api intercepts
        // VERSION
        // GENERAL
        if matches.opt_present("difficulty_adjustment") {
            api("difficulty_adjustment", "v9999", true);
            std::process::exit(0);
        }
        if matches.opt_present("prices") {
            api("prices", "v9999", true);
            std::process::exit(0);
        }
        if matches.opt_present("historical_price") {
            if matches.opt_present("currency") {
                //print!("currency={}\n", matches.opt_present("currency"));
                let currency = matches.opt_str("currency");
                //print!("currency={}", currency.clone().unwrap());
                if matches.opt_present("timestamp") {
                    //print!("timestamp={}\n", matches.opt_present("timestamp"));
                    let timestamp = matches.opt_str("timestamp");
                    historical_price(currency.as_ref().unwrap(), &timestamp.unwrap());
                } else {
                    historical_price(&currency.unwrap(), "");
                }
            } else {
                historical_price("", "");
            }

            //historical_prices(&"USD", &"1500000000");
            //historical_prices(&"EUR", &"1500000000");
            std::process::exit(0);
        }

        // ADDRESSES
        if matches.opt_present("address") {
            let address = matches.opt_str("address");
            //print!("address={:?}", address);
            api("address", &address.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("address_txs") {
            let address = matches.opt_str("address_txs");
            api("address_txs", &address.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("address_txs_chain") {
            let address = matches.opt_str("address_txs_chain");
            api("address_txs_chain", &address.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("address_txs_mempool") {
            let address = matches.opt_str("address_txs_mempool");
            api("address_txs_mempool", &address.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("address_utxo") {
            let address = matches.opt_str("address_utxo");
            api("address_utxo", &address.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("validate_address") {
            let validate_address = matches.opt_str("validate_address");
            api("validate_address", &validate_address.unwrap(), true);
            std::process::exit(0);
        }
        // BLOCKS
        if matches.opt_present("block") {
            let block = matches.opt_str("block");
            api("block", &block.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("block_header") {
            let block_header = matches.opt_str("block_header");
            api("block_header", &block_header.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("block_height") {
            let block_height = matches.opt_str("block_height");
            api("block_height", &block_height.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("blocks_timestamp") {
            let blocks_timestamp = matches.opt_str("blocks_timestamp");
            api("blocks_timestamp", &blocks_timestamp.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("block_raw") {
            let block_raw = matches.opt_str("block_raw");
            api("block_raw", &block_raw.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("block_status") {
            let block_status = matches.opt_str("block_status");
            api("block_status", &block_status.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("blocks_tip_height") {
            api("blocks_tip_height", "extraneous_arg", true);
            std::process::exit(0);
        }
        if matches.opt_present("blocks_tip_hash") {
            api("blocks_tip_hash", "extraneous_arg", true);
            std::process::exit(0);
        }
        if matches.opt_present("block_txid") {
            let arg_block_txid = matches.opt_str("block_txid");
            let arg_block_txindex = matches.opt_str("block_txindex");
            block_txid(&arg_block_txid.unwrap(), &arg_block_txindex.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_txids") {
            let arg_block_txids = matches.opt_str("block_txids");
            block_txids(&arg_block_txids.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_txs") {
            let arg_block_txs = matches.opt_str("block_txs");
            let arg_start_index = matches.opt_str("start_index");
            block_txs(&arg_block_txs.unwrap(), &arg_start_index.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks") {
            let arg_blocks = matches.opt_str("blocks");
            blocks(&arg_blocks.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("blocks_bulk") {
            let arg_min_height = matches.opt_str("min_height");
            let arg_max_height = matches.opt_str("max_height");
            blocks_bulk(&arg_min_height.unwrap(), &arg_max_height.unwrap(), true);
            std::process::exit(0);
        }
        // MINING
        if matches.opt_present("mining_pools") {
            let arg_timeperiod = matches.opt_str("timeperiod");
            api("mining_pools", &arg_timeperiod.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("mining_pool") {
            let arg_slug = matches.opt_str("slug");
            api("mining_pool", &arg_slug.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("mining_hashrate_pools") {
            let arg_timeperiod = matches.opt_str("timeperiod");
            api("mining_hashrate_pools", &arg_timeperiod.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("mining_pool_hashrate") {
            let arg_slug = matches.opt_str("slug");
            api("mining_hashrate_pool", &arg_slug.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("mining_pool_blocks") {
            let arg_slug = matches.opt_str("slug");
            let arg_blockheight = matches.opt_str("blockheight");
            mining_pool_blocks(&arg_slug.unwrap(), &arg_blockheight.unwrap());
            std::process::exit(0);
        }

        if matches.opt_present("difficulty_adjustments") {
            let arg_interval = matches.opt_str("interval");
            api("difficulty_adjustments", &arg_interval.unwrap(), true);
            std::process::exit(0);
        }

        if matches.opt_present("blocks_audit_scores") {
            let arg_blockheight = matches.opt_str("blockheight");
            api("blocks_audit_scores", &arg_blockheight.unwrap(), true);
            std::process::exit(0);
        }
        if matches.opt_present("block_audit_summary") {
            let arg_blockhash = matches.opt_str("blockhash");
            api("block_audit_summary", &arg_blockhash.unwrap(), true);
            std::process::exit(0);
        }

        if matches.opt_present("h")
            || (matches.free.is_empty()
                && !matches.opt_present("u")
                && !matches.opt_present("r")
                && !matches.opt_present("V")
                && !matches.opt_present("l")
                && !matches.opt_present("d")
                && !matches.opt_present("v")
                && std::io::stdin().is_terminal())
        {
            let usage = format!(
                "\n{} {} \u{2014} {}.\
                \n\u{221F} written by {}\
                \n\u{221F} licensed under MIT <{}>\
                \n\nUsage:\n    {} [options] <file(s)>",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_AUTHORS"),
                env!("CARGO_PKG_REPOSITORY"),
                "mempool-space",
            );
            println!("{}", opts.usage(&usage));
            process::exit(0)
        }

        if matches.opt_present("v") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0)
        }

        Args {
            config: env::var("RPASTE_CONFIG")
                .ok()
                .or_else(|| matches.opt_str("c"))
                .map(PathBuf::from),

            // invoke mempool-space_dashboard
            dashboard: matches.opt_str("dashboard"),

            // mempool api intercepts
            // mempool api version
            version: matches.opt_str("version"),

            // GENERAL
            difficulty_adjustment: matches.opt_str("difficulty_adjustment"),
            currency: matches.opt_str("currency"),
            prices: matches.opt_str("prices"),
            timestamp: matches.opt_str("timestamp"),
            historical_price: matches.opt_str("historical_price"),

            // ADDRESSES
            address: matches.opt_str("address"),
            address_txs: matches.opt_str("address_txs"),
            address_txs_chain: matches.opt_str("address_txs_chain"),
            address_txs_mempool: matches.opt_str("address_txs_mempool"),
            address_utxo: matches.opt_str("address_utxo"),
            validate_address: matches.opt_str("validate_address"),

            // BLOCK/S
            // https://mempool.space/api/block/<endpoint>
            // https://mempool.space/api/block/<block_hash>
            // BLOCK
            block: matches.opt_str("block"),
            // https://mempool.space/api/block/<block_hash>/header
            block_header: matches.opt_str("block_header"),
            // BLOCK_HEIGHT
            // https://mempool.space/api/block-height/615615
            block_height: matches.opt_str("block_height"),

            // V1 MINING BLOCKS TIMESTAMP
            // https://mempool.space/api/v1/mining/blocks/timestamp/<UTC_SECS>"
            blocks_timestamp: matches.opt_str("blocks_timestamp"),

            // BLOCK
            // https://mempool.space/api/block/<block_hash>/raw
            block_raw: matches.opt_str("block_raw"),
            // https://mempool.space/api/block/<block_hash>/status
            block_status: matches.opt_str("block_status"),

            // BLOCKS
            // BLOCKS TIP HEIGHT
            // https://mempool.space/api/blocks/tip/height
            blocks_tip_height: matches.opt_str("blocks_tip_height"),
            // BLOCKS TIP HASH
            // https://mempool.space/api/blocks/tip/hash
            blocks_tip_hash: matches.opt_str("blocks_tip_hash"),

            // BLOCK
            // BLOCK BLOCK_HASH TXID INDEX
            // https://mempool.space/api/block/<block_hash>/<txid>/<index>
            block_txid: matches.opt_str("block_txid"),
            block_txindex: matches.opt_str("block_txindex"),
            // BLOCK BLOCK_HASH TXIDS
            // https://mempool.space/api/block/<block_hash>/<txids>
            block_txids: matches.opt_str("block_txids"),
            // BLOCK BLOCK_HASH TXS
            // https://mempool.space/api/block/<block_hash>/<txs>
            block_txs: matches.opt_str("block_txs"),
            start_index: matches.opt_str("start_index"),

            // V1 BLOCKS
            // https://mempool.space/api/v1/blocks/<BLOCK_HEIGHT>"
            blocks: matches.opt_str("blocks"),

            // V1 BLOCKS_BULK
            // https://mempool.space/api/v1/blocks-bulk/<BLOCK_HEIGHT_START>/<BLOCK_HEIGHT_STOP>"
            blocks_bulk: matches.opt_str("blocks_bulk"),
            min_height: matches.opt_str("min_height"),
            max_height: matches.opt_str("max_height"),

            // MINING
            // V1 MINING POOLS TIMEPERIOD
            mining_pools: matches.opt_str("mining_pools"),
            timeperiod: matches.opt_str("timeperiod"),

            // V1 MINING POOL SLUG
            mining_pool: matches.opt_str("mining_pool"),
            slug: matches.opt_str("slug"),

            // V1 MINING DIFFICULTY_ADJUSTMENTS INTERVAL
            difficulty_adjustments: matches.opt_str("difficulty_adjustments"),
            interval: matches.opt_str("interval"),

            // V1 BLOCKS_AUDIT_SCORE BLOCK_HASH
            blocks_audit_score: matches.opt_str("blocks_audit_score"),
            // block_hash: matches.opt_str("block_hash"),

            // V1 BLOCKS_AUDIT_SCORE BLOCKHEIGHT
            blocks_audit_scores: matches.opt_str("blocks_audit_scores"),
            blockheight: matches.opt_str("blockheight"),

            // V1 BLOCK_AUDIT_SUMMARY BLOCKHEIGHT
            block_audit_summary: matches.opt_str("block_audit_summary"),
            blockhash: matches.opt_str("blockhash"),

            //OPTOPT
            server: matches.opt_str("s"),
            auth: matches.opt_str("a"),
            url: matches.opt_str("u"),
            remote: matches.opt_str("r"),
            oneshot: matches.opt_present("o"),
            expire: matches.opt_str("e"),
            prettify: matches.opt_present("p"),
            print_server_version: matches.opt_present("V"),
            list_files: matches.opt_present("l"),
            delete: matches.opt_present("d"),
            filename: matches.opt_str("n"),
            files: matches.free,
        }
    }
}
