use getopts::Options;
use std::env;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process;

use crate::api;
use crate::api::{api, blocking};

/// GET /api/v1/historical-price?currency=CURRENCY&timestamp=TIMESTAMP
///
/// <https://mempool.space/docs/api/rest#get-historical-price>
///
/// CURRENCY [USD, EUR, GBP, CAD, CHF, AUD, JPY]
///
/// TIMESTAMP [$(date +%s), 1231006505]
///
/// USAGE:
///
/// `mempool-space --historical_price --currency USD --timestamp $(date +%s)`
///
/// `mempool-space_historical_price USD $(date +%s)`
pub fn historical_price(currency: &str, timestamp: &str) {
    let _res = blocking(&format!(
        "v1/historical-price?currency={}&timestamp={}",
        &format!("{:}", &currency),
        &format!("{:}", &timestamp)
    ));
}
/// GET /api/block/:hash/txid/:index
/// <https://mempool.space/docs/api/rest#get-block-transaction-id>
pub fn block_txid(block_hash: &str, txindex: &str) {
    let _res = blocking(&format!("block/{}/txid/{}", block_hash, txindex));
}
/// GET /api/block/:hash/txids
/// <https://mempool.space/docs/api/rest#get-block-transaction-ids>
pub fn block_txids(block_hash: &str) {
    let _res = blocking(&format!("block/{}/txids", block_hash));
}
/// GET /api/block/:hash/txs[/:start_index] (start_index % 25 = 0)
/// <https://mempool.space/docs/api/rest#get-block-transactions>
pub fn block_txs(block_hash: &str, start_index: &str) {
    let start_index_int = start_index.parse::<i32>().unwrap_or(0);
    if start_index_int % 25 == 0 {
        let _res = blocking(&format!("block/{}/txs/{}", block_hash, start_index));
    } else {
        let _res = blocking(&format!("block/{}/txs/{}", block_hash, &"0"));
    }
}
/// GET /api/v1/blocks[/:startHeight]
/// <https://mempool.space/docs/api/rest#get-blocks>
pub fn blocks(start_height: &str) {
    let blocks_tip_height = api::api("blocks_tip_height", &"extraneous_arg");
    let blocks_tip_height_int = blocks_tip_height.parse::<i32>().unwrap_or(0);
    let start_height_int = start_height.parse::<i32>().unwrap_or(0);
    if start_height_int >= 0 && start_height_int <= blocks_tip_height_int {
        let _res = blocking(&format!("v1/blocks/{}", start_height));
    } else {
        let _res = blocking(&format!("v1/blocks"));
    }
}
/// GET /api/v1/blocks-bulk/:minHeight[/:maxHeight]
/// <https://mempool.space/docs/api/rest#get-blocks-bulk>
pub fn blocks_bulk(min_height: &str, max_height: &str) {
    let min_height_int = min_height.parse::<i32>().unwrap_or(0);
    let max_height_int = max_height.parse::<i32>().unwrap_or(0);
    if min_height_int >= 0 && max_height_int >= 0 && min_height_int < max_height_int {
        let _res = blocking(&format!("v1/blocks-bulk/{}/{}", min_height, max_height));
    } else if min_height_int >= 0 && max_height_int >= 0 && min_height_int >= max_height_int {
        let _res = blocking(&format!("v1/blocks-bulk/{}/{}", max_height, min_height));
    } else {
        let blocks_tip_height = api::api("blocks_tip_height", &"extraneous_arg");
        let _res = blocking(&format!("v1/blocks-bulk/{}/{}", min_height, blocks_tip_height));
    }
    print!("This API is disabled. Set config.MEMPOOL.MAX_BLOCKS_BULK_QUERY to a positive number to enable it.");
}

/// <https://mempool.space/docs/api/rest>
/// - [API/REST](https://mempool.space/docs/api/rest)
///     - [GENERAL](https://mempool.space/docs/api/rest#get-difficulty-adjustment)
///         - GET /api/v1/difficulty-adjustment \<<https://mempool.space/api/v1/difficulty-adjustment>\>
///         - GET /api/v1/prices \<<https://mempool.space/api/v1/prices>\>
///         - GET /api/v1/historical-price?currency=EUR&timestamp=1500000000 \<<https://mempool.space/api/v1/historical-price?currency=EUR&timestamp=1500000000>\>
///     - [ADDRESSES](https://mempool.space/docs/api/rest#get-address)
///         - GET /api/address/:address \<<https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv>\>
///         - GET /api/address/:address/txs \<<https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs>\>
///         - GET /api/address/:address/txs/chain \<<https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/chain>\>
///         - GET /api/address/:address/txs/mempool \<<https://mempool.space/api/address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/mempool>\>
///         - GET /api/address/:address/utxo \<<https://mempool.space/api/address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY/utxo>\>
///         - GET /api/v1/validate-address/:address \<<https://mempool.space/api/v1/validate-address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY>\>
///     - [BLOCKS](https://mempool.space/docs/api/rest#get-block)
///         - GET /api/block/:hash \<<https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce>\>
///         - GET /api/block/:hash/header \<<https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/header>\>
///         - GET /api/block-height/:height \<<https://mempool.space/api/block-height/615615>\>
///         - GET /api/v1/mining/blocks/timestamp/:timestamp \<<https://mempool.space/api/v1/mining/blocks/timestamp/1672531200>\>
///         - GET /api/block/:hash/raw \<<https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/raw>\>
///         - GET /api/block/:hash/status \<<https://mempool.space/api/block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2/status>\>
///         - GET /api/blocks/tip/height \<<https://mempool.space/api/blocks/tip/height>\>
///         - GET /api/blocks/tip/hash \<<https://mempool.space/api/blocks/tip/hash>\>
///         - GET /api/block/:hash/txid/:index \<<https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txid/218>\>
///         - GET /api/block/:hash/txids \<<https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txids>\>
///         - GET /api/block/:hash/txs[/:start_index] \<<https://mempool.space/api/block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs>\>
///         - GET /api/v1/blocks[/:startHeight] \<<https://mempool.space/api/v1/blocks/730000>\>
///         - GET /api/v1/blocks-bulk/:minHeight[/:maxHeight] \<<https://mempool.space/api/v1/blocks-bulk/100000/100000>\> (Enterprise)
///     - [MINING](https://mempool.space/docs/api/rest#get-mining-pools)
///         - GET /api/v1/mining/pools[/:timePeriod] \<<https://mempool.space/api/v1/mining/pools/1w>\>
///         - GET /api/v1/mining/pool/:slug \<<https://mempool.space/api/v1/mining/pool/antpool>\>
///         - GET /api/v1/mining/hashrate/pools/[\:timePeriod] \<<https://mempool.space/api/v1/mining/hashrate/pools/1m>\>
///         - GET /api/v1/mining/pool/:slug/hashrate \<<https://mempool.space/api/v1/mining/pool/foundryusa/hashrate>\>
///         - GET /api/v1/mining/pool/:slug/blocks/[\:blockHeight] \<<https://mempool.space/api/v1/mining/pool/luxor/blocks/730000>\>
///         - GET /api/v1/mining/hashrate/[\:timePeriod] \<<https://mempool.space/api/v1/mining/hashrate/3d>\>
///         - GET /api/v1/mining/pool/:slug/blocks/[\:blockHeight] \<<https://mempool.space/api/v1/mining/pool/luxor/blocks/730000>\>
///         - GET /api/v1/mining/difficulty-adjustments/[\:interval] \<<https://mempool.space/api/v1/mining/difficulty-adjustments/1m>\>
///         - GET /api/v1/mining/reward-stats/:blockCount \<<https://mempool.space/api/v1/mining/reward-stats/100>\>
///         - GET /api/v1/mining/blocks/fees/:timePeriod \<<https://mempool.space/api/v1/mining/blocks/fees/1w>\>
///         - GET /api/v1/mining/blocks/rewards/:timePeriod \<<https://mempool.space/docs/api/rest#get-block-rewards>\>
///         - GET /api/v1/mining/blocks/fee-rates/:timePeriod \<<https://mempool.space/api/v1/mining/blocks/fee-rates/1m>\>
///         - GET /api/v1/mining/blocks/sizes-weights/:timePeriod \<<https://mempool.space/api/v1/mining/blocks/sizes-weights/3y>\>
///         - GET /api/v1/mining/blocks/predictions/:timePeriod \<<https://mempool.space/api/v1/mining/blocks/predictions/3y>\>
///         - GET /api/v1/mining/blocks/audit/score/:blockHash \<<https://mempool.space/api/v1/mining/blocks/audit/score/000000000000000000032535698c5b0c48283b792cf86c1c6e36ff84464de785>\>
///         - GET /api/v1/mining/blocks/audit/scores/:startHeight \<<https://mempool.space/api/v1/mining/blocks/audit/scores/820000>\>
///         - GET /api/v1/block/:blockHash/audit-summary \<<https://mempool.space/api/v1/block/00000000000000000000f218ceda7a5d9c289040b9c3f05ef9f7c2f4930e0123/audit-summary>\>
///     - [FEES](https://mempool.space/docs/api/rest#get-mempool-blocks-fees)
///         - GET /api/v1/fees/mempool-blocks \<<https://mempool.space/api/v1/fees/mempool-blocks>\>
///         - GET /api/v1/fees/recommended \<<https://mempool.space/api/v1/fees/recommended>\>
///     - [MEMPOOL](https://mempool.space/docs/api/rest#get-mempool)
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///     - [TRANSACTIONS](https://mempool.space/docs/api/rest#get-cpfp)
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///     - [LIGHTNING](https://mempool.space/docs/api/rest#get-lightning-network-stats)
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///     - [ACCELERATOR (Public)](https://mempool.space/docs/api/rest#accelerator-estimate)
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///     - [ACCELERATOR (Authenticated)](https://mempool.space/docs/api/rest#accelerator-top-up-history)
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///         - GET \<< >\>
///
///         USAGE:
///       - mempool-space \--difficulty_adjustment (flagged)
///       - mempool-space_difficulty_adjustment (executable)
///         1. Flags follow the mempool.space api/rest (replace dashes with underscores)
///         2. Flags invoke the executable
///
#[derive(Debug, Default)]
pub struct Args {
    // VERSION
    // pub version: Option<String>,
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

    /// - ADDRESSES
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

    /// - BLOCK
    /// `https://mempool.space/api/block/<BLOCK_HASH>`
    pub block: Option<String>,

    /// - BLOCK_HEADER
    /// `https://mempool.space/api/block/<BLOCK_HASH>/header`
    pub block_header: Option<String>,

    /// - BLOCK_HEIGHT
    /// `https://mempool.space/api/block-height/<BLOCK_HEIGHT>`
    pub block_height: Option<String>,

    /// - V1 MINING BLOCKS TIMESTAMP <UTC_SECS>
    /// `https://mempool.space/api/v1/mining/blocks/timestamp/<UTC_SECS>`
    pub blocks_timestamp: Option<String>,

    /// - BLOCK
    /// `https://mempool.space/api/block/<BLOCK_HASH>/raw`
    pub block_raw: Option<String>,
    /// `https://mempool.space/api/block/<BLOCK_HASH>/status`
    pub block_status: Option<String>,

    /// - BLOCKS TIP HEIGHT
    /// `https://mempool.space/api/blocks/tip/height`
    pub blocks_tip_height: Option<String>,
    /// - BLOCKS TIP HASH
    /// `https://mempool.space/api/blocks/tip/hash`
    pub blocks_tip_hash: Option<String>,

    /// `https://mempool.space/api/block/<BLOCK_HASH>/<TXINDEX>`
    ///
    /// mempool-space --block_txid \<BLOCK_HASH\> --block_txindex \<INT\>
    ///
    /// mempool-space_block_txid \<BLOCK_HASH\> \<TXINDEX\>
    ///
    pub block_txid: Option<String>,
    ///
    pub block_txindex: Option<String>,

    /// - BLOCK \<BLOCK_HASH\> \<TXIDS\>
    /// `https://mempool.space/api/block/<TXID>`
    pub block_txids: Option<String>,

    /// - BLOCK \<BLOCK_HASH\> \<TXS\>
    /// `https://mempool.space/api/block/<BLOCK_HASH>/txs`
    pub block_txs: Option<String>,
    /// mempool-space --block_txs \<BLOCK_HASH\> --start_index \<START_INDEX\>
    pub start_index: Option<String>,

    /// - V1 BLOCKS \<BLOCK_HEIGHT\>
    /// `https://mempool.space/api/v1/blocks/<BLOCKS_START_HEIGHT>`
    pub blocks: Option<String>,

    /// - V1 BLOCKS_BULK \<MAX_HEIGHT\> \<MIN_HEIGHT\>
    /// `https://mempool.space/api/v1/blocks-bulk/<MIN_HEIGHT>/<MAX_HEIGHT>`
    pub blocks_bulk: Option<String>,
    /// `https://mempool.space/api/v1/blocks-bulk/<MIN_HEIGHT>/<MAX_HEIGHT>`
    pub min_height: Option<String>,
    /// `https://mempool.space/api/v1/blocks-bulk/<MIN_HEIGHT>/<MAX_HEIGHT>`
    pub max_height: Option<String>,

    /// - V1 BLOCKS_AUDIT_SCORE \<BLOCK_HASH\>
    /// `https://mempool.space/api/v1/mining/blocks/audit/score/<BLOCK_HASH>`
    pub blocks_audit_score: Option<String>,
    /// blockhash
    pub blockhash: Option<String>,

    /// - V1 BLOCKS_AUDIT_SCORES \<BLOCKHEIGHT\>
    /// `https://mempool.space/api/v1/mining/blocks/audit/scores/<BLOCKHEIGHT>`
    pub blocks_audit_scores: Option<String>,
    /// blockheight
    pub blockheight: Option<String>,

    /// - V1 BLOCK_AUDIT_SUMMARY \<BLOCKHASH\>
    /// `https://mempool.space/api/v1/mining/block/<BLOCKHASH>/audit-summary`
    pub block_audit_summary: Option<String>,
    // /// blockhash
    // pub blockhash: Option<String>,

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
        opts.optflag("v", "vv", "prints version information");
        opts.optflag("V", "server-version", "retrieves the server version");
        opts.optflag("l", "list", "lists files on the server");
        opts.optflag("d", "delete", "delete files from server");
        opts.optflag("o", "oneshot", "generates one shot links");
        opts.optflag("p", "pretty", "prettifies the output");

        //mempool api intercepts
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

        //MINING
        //
        opts.optflag("", "blocks_audit_score", "blocks_audit_score api call");
        opts.optopt("", "block_hash", "blocks_audit_score api call", "BLOCK_HASH");

        opts.optflag("", "blocks_audit_scores", "blocks_audit_scores api call");
        //opts.optopt("", "blockheight", "blocks_audit_scores api call", "BLOCKHEIGHT");

        opts.optflag("", "block_audit_summary", "block_audit_summary api call");
        opts.optopt("", "blockhash", "block_audit_summary api call", "BLOCKHASH");

        //OPTOPT
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

        //mempool api intercepts
        // VERSION
        // GENERAL
        if matches.opt_present("difficulty_adjustment") {
            api("difficulty_adjustment", &"v9999");
            std::process::exit(0);
        }
        if matches.opt_present("prices") {
            api("prices", &"v9999");
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
                    historical_price(&currency.as_ref().unwrap(), &timestamp.unwrap());
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
            api("address", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_txs") {
            let address = matches.opt_str("address_txs");
            api("address_txs", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_txs_chain") {
            let address = matches.opt_str("address_txs_chain");
            api("address_txs_chain", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_txs_mempool") {
            let address = matches.opt_str("address_txs_mempool");
            api("address_txs_mempool", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_utxo") {
            let address = matches.opt_str("address_utxo");
            api("address_utxo", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("validate_address") {
            let validate_address = matches.opt_str("validate_address");
            api("validate_address", &validate_address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block") {
            let block = matches.opt_str("block");
            api("block", &block.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_header") {
            let block_header = matches.opt_str("block_header");
            api("block_header", &block_header.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_height") {
            let block_height = matches.opt_str("block_height");
            api("block_height", &block_height.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks_timestamp") {
            let blocks_timestamp = matches.opt_str("blocks_timestamp");
            api("blocks_timestamp", &blocks_timestamp.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_raw") {
            let block_raw = matches.opt_str("block_raw");
            api("block_raw", &block_raw.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_status") {
            let block_status = matches.opt_str("block_status");
            api("block_status", &block_status.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks_tip_height") {
            api("blocks_tip_height", &"extraneous_arg");
            std::process::exit(0);
        }
        if matches.opt_present("blocks_tip_hash") {
            api("blocks_tip_hash", &"extraneous_arg");
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
            blocks(&arg_blocks.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks_bulk") {
            let arg_min_height = matches.opt_str("min_height");
            let arg_max_height = matches.opt_str("max_height");
            blocks_bulk(&arg_min_height.unwrap(), &arg_max_height.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks_audit_score") {
            let arg_block_hash = matches.opt_str("block_hash");
            api("blocks_audit_score", &arg_block_hash.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("blocks_audit_scores") {
            let arg_blockheight = matches.opt_str("blockheight");
            api("blocks_audit_scores", &arg_blockheight.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block_audit_summary") {
            let arg_blockhash = matches.opt_str("blockhash");
            api("block_audit_summary", &arg_blockhash.unwrap());
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

            // mempool api intercepts
            // mempool api version
            // version: matches.opt_str("version"),

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
