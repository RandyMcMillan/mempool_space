
## cargo install:

``cargo install mempool_space``

## cargo install --git:

``cargo install --git https:github.com/RandyMcMillan/mempool_space.git``



### [mempool-space](src/bin/mempool-space.rs) --option arg<sub>1</sub> ... --option arg<sub>n</sub>

	mempool-space --option

	mempool-space --option arg

	mempool-space --option arg --option arg

### [mempool-space_option](src/bin/) arg<sub>1</sub> ... arg<sub>n</sub>

	mempool-space_option

	mempool-space_option arg

	mempool-space_option arg arg

### [mempool-space_option_string](src/bin/) arg<sub>1</sub> ... arg<sub>n</sub>

	mempool-space_option_string

	mempool-space_option_string arg

	mempool-space_option_string arg arg




## GENERAL




	mempool-space --difficulty_adjustment

	mempool-space_difficulty_adjustment

	mempool-space --prices

	mempool-space_prices

	mempool-space --historical_price --currency [USD, CAD, GBP, CHF, AUD, JPY] --timestamp utc_sec

	mempool-space --historical_price --currency EUR --timestamp 1500000000

	mempool-space --historical_price --currency USD --timestamp $(date +%s)




## ADDRESSES




	mempool-space --address 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space_address 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space --address_txs 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space_address_txs 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space --address_txs_chain 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space_address_txs_chain 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space --address_txs_mempool 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space_address_txs_mempool 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space --address_utxo 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space_address_utxo 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv

	mempool-space --validate_address 1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY

	mempool-space_validate_address 1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY



## BLOCKS




	mempool-space --block 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce

	mempool-space_block 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce

	mempool-space --block_header 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2

	mempool-space_block_header 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2

	mempool-space --block_height 615615

	mempool-space_block_height 615615

	mempool-space --blocks_timestamp 1672531200

	mempool-space_blocks_timestamp 1672531200

	mempool-space --block_raw 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2

	mempool-space_block_raw 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2

	mempool-space --block_status 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2

	mempool-space_block_status 0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2

	mempool-space --blocks_tip_height

	mempool-space_blocks_tip_height

	mempool-space --blocks_tip_hash

	mempool-space_blocks_tip_hash

	mempool-space --block_txid 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce --block_txindex 218

	mempool-space_block_txid 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce 218

	mempool-space --block_txids 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce

	mempool-space_block_txids 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce

	mempool-space --block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce --start_index 0

	mempool-space --block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce --start_index 25

	mempool-space_block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce 0

	mempool-space_block_txs 000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce 25

	mempool-space --blocks 730000

	mempool-space_blocks 730000

	mempool-space --blocks_bulk --min_height 730000 --max_height 840000

	mempool-space_blocks_bulk 730000 840000



## MINING




	mempool-space --mining_pools --timeperiod [24h 3d 1w 1m 3m 6m 1y 2y 3y]

	mempool-space_mining_pools [24h 3d 1w 1m 3m 6m 1y 2y 3y]

	mempool-space --mining_pool --slug unknown

	mempool-space_mining_pool unknown

	mempool-space --mining_hashrate_pools --timeperiod [1m, 3m, 6m, 1y, 2y, 3y]

	mempool-space_mining_hashrate_pools 1m

	mempool-space --mining_pool_hashrate --slug foundryusa

	mempool-space_mining_pool_hashrate foundryusa

	mempool-space --mining_pool_hashrate --slug unknown

	mempool-space_mining_pool_hashrate unknown

	more

	mempool-space --mining_pool_blocks --slug luxor --blockheight 730000

	mempool-space_mining_pool_blocks luxor 730000

	mempool-space --blocks_audit_score --block_hash 00000000000000000002352696778fc14532ccb923fde167fc754de26e6adbcd

	mempool-space_blocks_audit_score 00000000000000000002352696778fc14532ccb923fde167fc754de26e6adbcd

	mempool-space --blocks_audit_scores --blockheight 820000

	mempool-space_blocks_audit_scores 820000

	mempool-space --block_audit_summary --blockhash 00000000000000000000f218ceda7a5d9c289040b9c3f05ef9f7c2f4930e0123

	mempool-space_block_audit_summary 00000000000000000000f218ceda7a5d9c289040b9c3f05ef9f7c2f4930e0123



## FEES



	mempool-space_fees_mempool_blocks

	mempool-space --fees_mempool_blocks

	mempool-space_fees_recommended

	mempool-space --fees_recommended



## MEMPOOL




	mempool-space_mempool

	mempool_space --mempool

	mempool-space_mempool_full_rbf_transactions

	mempool-space --mempool_full_rbf_transactions

	mempool-space_mempool_rbf_transactions

	mempool-space --mempool_rbf_transactions

	mempool-space_mempool_recent

	mempool-space --mempool_recent

	mempool-space_mempool_txids

	mempool-space --mempool_txids





## TRANSACTIONS







## LIGHTNING







## ACCELERATOR PUBLIC







## ACCELERATOR AUTHENTICATED




#![warn(missing_docs, clippy::unwrap\_used)]

#[warn(missing_docs, clippy::unwrap\_used)]
### 	pub mod api
pub mod api;
### 	pub mod error
pub mod error;
### 	pub mod resolve_policy
pub mod resolve_policy;
### 	pub mod target
pub mod target;

pub use error::{CheckTargetError, ParseTargetError, ResolveTargetError};

pub use resolve_policy::ResolvePolicy;

pub use target::{Fqhn, IcmpTarget, Port, Status, Target, TcpTarget};


### 	Command-line argument parser.
pub mod args;


#[cfg(feature = "async")]
### 	pub mod async_target;
pub mod async_target;

#[cfg(feature = "async")]
### 	pub use async_target::{AsyncTarget, AsyncTargetExecutor, BoxedHandler, BoxedTarget, OldStatus};
pub use async_target::{AsyncTarget, AsyncTargetExecutor, BoxedHandler, BoxedTarget, OldStatus};

### 	Configuration file parser.
pub mod config;

### 	Custom error implementation.
pub mod this_error;

### 	Upload handler.
pub mod upload;

use crate::args::Args;

use crate::config::Config;

use crate::this_error::{Error, Result};

use crate::upload::Uploader;

use crossterm::style::Stylize;

use std::fs;

use std::io::IsTerminal;

use std::io::Read;

use std::io::{self};


###  const CONFIG_FILE: &str = "config.toml";

const CONFIG_FILE: &str = "config.toml";

### 	pub fn run(args: Args) -> Result<()>

pub fn run(args: Args) -> Result<()> {
    
    let mut config = Config::default();
    

    if let Some(ref config_path) = args.config {
        
        config = toml::from_str(&fs::read_to_string(config_path)?)?
    
    } else {
        
        for path in [
            dirs_next::home_dir().map(|p| p.join(".mempool").join(CONFIG_FILE)),
            dirs_next::home_dir().map(|p| p.join(".config").join(CONFIG_FILE)),
            dirs_next::config_dir().map(|p| p.join(".config").join(CONFIG_FILE)),
        ]
        .iter()
        .filter_map(|v| v.as_ref())
        {
            if path.exists() {
                config = toml::from_str(&fs::read_to_string(path)?)?;
                break;
            }
        }
    }
    config.update_from_args(&args);

    if config.server.address.is_empty() {
        return Err(Error::NoServerAddressError);
    }

    let uploader = Uploader::new(&config);

    if args.print_server_version {
        println!("TODO: {}", uploader.retrieve_version()?.trim());

        return Ok(());
    }

    if args.list_files {
        let prettify = args.prettify || config.style.as_ref().map(|style| style.prettify).unwrap_or(false);

        uploader.retrieve_list(&mut io::stdout(), prettify)?;

        return Ok(());
    }

    let mut results = Vec::new();

    if let Some(ref url) = args.url {
        TODO TOR_URL etc...
        results.push(uploader.upload_url(url));
    } else if let Some(ref remote_url) = args.remote {
        results.push(uploader.upload_remote_url(remote_url));
    } else if !std::io::stdin().is_terminal() || args.files.contains(&String::from("-")) {
        let mut buffer = Vec::new();

        let mut stdin = io::stdin();

        stdin.read_to_end(&mut buffer)?;

        results.push(uploader.upload_stream(&*buffer));
    } else {
        for file in args.files.iter() {
            if !args.delete {
                results.push(uploader.upload_file(file))
            } else {
                results.push(uploader.delete_file(file))
            }
        }
    }
    let prettify = args.prettify || config.style.as_ref().map(|style| style.prettify).unwrap_or(false);

    let format_padding = prettify
        .then(|| results.iter().map(|v| v.0.len()).max())
        .flatten()
        .unwrap_or(1);

    for (data, result) in results.iter().map(|v| (v.0, v.1.as_ref())) {
        let data = if prettify {
            format!(
                "{:p$} {} ",
                data,
                if result.is_ok() {
                    "=>".green().bold()
                } else {
                    "=>".red().bold()
                },
                p = format_padding,
            )
        } else {
            String::new()
        };
        match result {
            Ok(url) => println!("{}{}", data, url.trim()),
            Err(e) => eprintln!("{data}{e}"),
        }
    }

    Ok(())
}


### 	pub fn reachable

pub fn reachable() -> bool {
    
    use api::{blocking, URL};
    
    use std::time::Instant;
    
    let start = Instant::now();
    
    let blocks_tip_height = String::from("blocks/tip/height");
    
    let res = blocking(&blocks_tip_height);
    
    let mut reachable = false;
    
    if !res.unwrap().to_string().is_empty() {
        
        println!(
            "\n\n{:?}:\nGET {}/{:} {:?}\n\n",
            start,
            URL[0],
            res.unwrap(),
            start.elapsed()
        );
        reachable = true;
    }
    
    let start = Instant::now();
    
    let blocks_tip_height = String::from("blocks_tip_height");
    
    let res = api::api(&blocks_tip_height, "", false);
    
    if !res.is_empty() && reachable {
        println!("\n\n{:?}:\nAPI {}/{:} {:?}\n\n", start, URL[0], res, start.elapsed());
        reachable = true;
    } else {
        reachable = false;
    }
    
    
    reachable
}


### 	pub fn wait(sleep: &str)

pub fn wait(sleep: &str) {
    
    use std::process::Command;
    
    let sleep_cmd = Command::new("sleep").arg(sleep).output().expect("wait:sleep failed");
    
    let _result = String::from_utf8(sleep_cmd.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    
    println!();
}


### 	cargo test -- --nocapture


#[cfg(test)]
mod tests {

    ###  TESTS
    use super::*;
    use crate::api::{api, blocking};

    ###  REACHABLE

    #[test]
    fn test_reachable() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        wait("1");
    }
    #[test]
    #[should_panic]
    fn test_reachable_panic() {
        let reachable = reachable();
        assert_eq!(reachable, false);
        wait("1");
    }

    ###  GENERAL

    #[test]
    fn test_difficulty_adjustment() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/v1/difficulty-adjustment
        let binding = format!("v1/difficulty-adjustment").clone();
        let get_difficulty_adjustment: &str = blocking(&binding).expect("REASON");
        let get_difficulty_adjustment = api("difficulty_adjustment", "extraneous_arg", true);
        wait("1");
    }
    #[test]
    fn test_price() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/v1/prices
        let binding = format!("v1/prices").clone();
        let get_prices: &str = blocking(&binding).expect("REASON");
        let get_prices = api("prices", "extraneous_arg", true);
        wait("1");
    }
    #[test]
    fn test_historical_price() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        use crate::args::historical_price;
         GET /api/v1/historical-price?currency=EUR&timestamp=1500000000
        let get_historical_price = historical_price(&"EUR", &"1500000000");
        let get_historical_prices = api("historical_price", "USD", true);
        wait("1");
    }

    ###  ADDRESSES TESTS

    #[test]
    fn test_address() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/address/:address
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv").clone();
        let get_address: &str = blocking(&binding).expect("test_address failed");
        let get_address = api("address", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv", true);
        wait("1");
    }
    #[test]
    fn test_address_txs() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/address/:address/txs
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs").clone();
        let get_address_txs: &str = blocking(&binding).expect("test_address_txs failed");
        let get_address_txs = api("address_txs", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv", true);
        wait("1");
    }
    #[test]
    fn test_address_txs_chain() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/address/:address/txs/chain
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/chain").clone();
        let get_address_txs_chain: &str = blocking(&binding).expect("REASON");
        let get_address_txs_chain = api("address_txs_chain", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv", true);
        wait("1");
    }
    #[test]
    fn test_address_txs_mempool() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/address/:address/txs/mempool
        let binding = format!("address/1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv/txs/mempool").clone();
        let get_address_txs_mempool: &str = blocking(&binding).expect("REASON");
        let get_address_txs_mempool = api("address_txs_mempool", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv", true);
        wait("1");
    }
    #[test]
    fn test_address_utxo() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/address/:address/utxo
        let binding = format!("address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY/utxo").clone();
        let get_address_utxo: &str = blocking(&binding).expect("existing valid address needed");
        let get_address_utxo = api("address_utxo", "1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv", true);
        wait("1");
    }
    #[test]
    fn test_validate_address() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/v1/validate-address/:address
        let binding = format!("v1/validate-address/1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY").clone();
        let get_valid_address: &str = blocking(&binding).expect("valid address needed");
        wait("1");
    }

    ###  BLOCKS TESTS

    #[test]
    fn test_block() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/block/:hash
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce").clone();
        let get_block: &str = blocking(&binding).expect("an existing block hash is needed");
        let get_block = api(
            "block",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
            true,
        );
        wait("1");
    }
    #[test]
    fn test_block_header() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/block/:hash/header
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce").clone();
        let get_block_header: &str = blocking(&binding).expect("an existing block hash is needed");
        let get_block_header = api(
            "block_header",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
            true,
        );
        wait("1");
    }
    #[test]
    fn test_block_height() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/block-height:height
        let binding = format!("block-height/615615").clone();
        let get_block_height: &str = blocking(&binding).expect("an existing block hash is needed");
        let get_block_height = api("block_height", "615615", true);
        wait("1");
    }
    #[test]
    fn test_blocks_timestamp() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/v1/mining/blocks/timestamp/:timestamp
        let binding = format!("v1/mining/blocks/timestamp/1672531200").clone();
        let get_timestamp: &str = blocking(&binding).expect("an existing block hash is needed");
        let get_timestamp = api("blocks_timestamp", "1672531200", true);
        wait("1");
    }
    #[test]
    fn test_block_raw() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/block/:hash/raw
        let binding = format!("block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2").clone();
        let get_block_raw: &str = blocking(&binding).expect("an existing block hash is needed");
        let get_block_raw = api(
            "block_raw",
            "0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2",
            true,
        );
        wait("1");
    }
    #[test]
    fn test_block_status() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/block/:hash/status
        let binding = format!("block/0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2").clone();
        let get_block_status: &str = blocking(&binding).expect("an existing block hash is needed");
        let get_block_status = api(
            "block_status",
            "0000000000000000000065bda8f8a88f2e1e00d9a6887a43d640e52a4c7660f2",
            true,
        );
        wait("1");
    }
    #[test]
    fn test_blocks_tip_height() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/blocks/tip/height
        let binding = format!("blocks/tip/height").clone();
        let get_blocks_tip_height: &str = blocking(&binding).expect("returns current block_height");
        let get_blocks_tip_height = api("blocks_tip_height", "extraneous_arg", true);
        wait("1");
    }
    #[test]
    fn test_blocks_tip_hash() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/blocks/tip/hash
        let binding = format!("blocks/tip/hash").clone();
        let get_blocks_tip_hash: &str = blocking(&binding).expect("returns current block/tip/hash");
        let get_block_tip_hash = api("blocks_tip_hash", "extraneous_arg", true);
        wait("1");
    }
    #[test]
    fn test_block_txid() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/block/:hash/txid/:index
        let binding =
            format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txid/218").clone();
        let get_block_txid: &str = blocking(&binding).expect("returns current txid from block index");
        let get_block_txid = api(
            "block_txid",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txid/218",
            true,
        );
        use crate::args::block_txid;
        block_txid(
            &"000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce",
            &"218",
        );
        wait("1");
    }
    #[test]
    fn test_block_txids() {
        let reachable = reachable();
        assert_eq!(reachable, true);
         GET /api/block/:hash/txids
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txids").clone();
        let get_block_txids: &str = blocking(&binding).expect("returns current txids from block");
        let get_block_txids = api(
            "block_txid",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txids",
            true,
        );
        use crate::args::block_txids;
        block_txids(&"000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce");
        wait("1");
    }
    #[test]
    fn test_block_txs() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        let binding = format!("block/000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs").clone();
        let get_block_txs: &str = blocking(&binding).expect("returns current txids from block");
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs",
            true,
        );
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs/0",
            true,
        );
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs/1", test if start_index_int % 25 == 0
            true,
        );
        let get_block_txs = api(
            "block_txs",
            "000000000000000015dc777b3ff2611091336355d3f0ee9766a2cf3be8e4b1ce/txs/25",
            true,
        );
    }
    #[test]
    fn test_blocks() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        let binding = format!("v1/blocks/730000").clone();
        let get_block_txid: &str = blocking(&binding).expect("returns current txid from block index");
        let get_block_txid = api("blocks", "730000", true);
        let get_blocks_tip_height = api("blocks_tip_height", "extraneous_arg", true);
        use crate::args::blocks;
        blocks(&"", true);
        wait("1");
        blocks(&"0", true);
        wait("1");
        blocks(&"25", true);
        wait("1");
        blocks(&"730000", true);
        wait("1");
        blocks(&get_blocks_tip_height, true);
        wait("1");
    }
    #[test]
    #[should_panic]
    fn test_blocks_bulk() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        let binding = format!("v1/blocks-bulk/730000/840000").clone();
        let get_block_bulk: &str = blocking(&binding).expect("returns current txid from block index");
        let get_block_bulk = api("blocks_bulk", "730000/840000", true);
        let get_blocks_tip_height = api("blocks_tip_height", "extraneous_arg", true);
        use crate::args::blocks_bulk;
        blocks_bulk(&"0", &"0", true);
        wait("1");
        blocks_bulk(&"0", &"1", true);
        wait("1");
        blocks_bulk(&"730000", &"840000", true);
        wait("1");
        blocks_bulk(&"730000", &get_blocks_tip_height, true);
        wait("1");
    }

    ###  MINING TESTS

    #[test]
    fn test_mining_pools() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  [24h 3d 1w 1m 3m 6m 1y 2y 3y]
        let binding = format!("v1/mining/pools/24h").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/1d");
        let get_mining_pools = api("mining_pools", "24h", true);
        wait("1");
        let binding = format!("v1/mining/pools/1d").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/1d");
        let get_mining_pools = api("mining_pools", "1d", true);
        wait("1");
        let binding = format!("v1/mining/pools/3d").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/3d");
        let get_mining_pools = api("mining_pools", "3d", true);
        wait("1");
        let binding = format!("v1/mining/pools/1w").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/1w");
        let get_mining_pools = api("mining_pools", "1w", true);
        wait("1");
        let binding = format!("v1/mining/pools/1m").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/1m");
        let get_mining_pools = api("mining_pools", "1m", true);
        wait("1");
        let binding = format!("v1/mining/pools/3m").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/3m");
        let get_mining_pools = api("mining_pools", "3m", true);
        wait("1");
        let binding = format!("v1/mining/pools/6m").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/6m");
        let get_mining_pools = api("mining_pools", "6m", true);
        wait("1");
        let binding = format!("v1/mining/pools/1y").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/1y");
        let get_mining_pools = api("mining_pools", "1y", true);
        wait("1");
        let binding = format!("v1/mining/pools/2y").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/2y");
        let get_mining_pools = api("mining_pools", "2y", true);
        wait("1");
        let binding = format!("v1/mining/pools/3y").clone();
        let get_mining_pools: &str = blocking(&binding).expect("returns current v1/mining/pools/3y");
        let get_mining_pools = api("mining_pools", "3y", true);
        wait("1");
    }
    #[test]
    fn test_mining_pool() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  antpool
        let binding = format!("v1/mining/pool/antpool").clone();
        let get_mining_pool: &str = blocking(&binding).expect("returns current v1/mining/pool/:slug");
        let get_mining_pool = api("mining_pool", "antpool", true);
        wait("1");

        ###  ...

        ###  unknown
        let binding = format!("v1/mining/pool/unknown").clone();
        let get_mining_pool: &str = blocking(&binding).expect("returns current v1/mining/pool/:slug");
        let get_mining_pool = api("mining_pool", "unknown", true);
        wait("1");
    }
    #[test]
    fn test_mining_hashrate_pools() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  timeperiod unspecified
        let binding = format!("v1/mining/hashrate/pools").clone();
        let mining_hashrate_pools: &str =
            blocking(&binding).expect("returns current v1/mining/hashrate/pools/[:timePeriod]");
        let get_mining_hashrate_pools = api("mining_hashrate_pools", "1m", true);
        wait("1");
          [1m, 3m, 6m, 1y, 2y, 3y]
        let binding = format!("v1/mining/hashrate/pools/1m").clone();
        let get_mining_hashrate_pools: &str =
            blocking(&binding).expect("returns current v1/mining/hashrate/pools/[:timePeriod]");
        let get_mining_hashrate_pools = api("mining_hashrate_pools", "1m", true);
        wait("1");
        let binding = format!("v1/mining/hashrate/pools/3m").clone();
        let get_mining_hashrate_pools: &str =
            blocking(&binding).expect("returns current v1/mining/hashrate/pools/[:timePeriod]");
        let get_mining_hashrate_pools = api("mining_hashrate_pools", "3m", true);
        wait("1");
        let binding = format!("v1/mining/hashrate/pools/6m").clone();
        let get_mining_hashrate_pools: &str =
            blocking(&binding).expect("returns current v1/mining/hashrate/pools/[:timePeriod]");
        let get_mining_hashrate_pools = api("mining_hashrate_pools", "6m", true);
        wait("1");
        let binding = format!("v1/mining/hashrate/pools/1y").clone();
        let get_mining_hashrate_pools: &str =
            blocking(&binding).expect("returns current v1/mining/hashrate/pools/[:timePeriod]");
        let get_mining_hashrate_pools = api("mining_hashrate_pools", "1y", true);
        wait("1");
        let binding = format!("v1/mining/hashrate/pools/2y").clone();
        let get_mining_hashrate_pools: &str =
            blocking(&binding).expect("returns current v1/mining/hashrate/pools/[:timePeriod]");
        let get_mining_hashrate_pools = api("mining_hashrate_pools", "2y", true);
        wait("1");
        let binding = format!("v1/mining/hashrate/pools/3y").clone();
        let get_mining_hashrate_pools: &str =
            blocking(&binding).expect("returns current v1/mining/hashrate/pools/[:timePeriod]");
        let get_mining_hashrate_pools = api("mining_hashrate_pools", "3y", true);
        wait("1");
    }
    #[test]
    fn test_mining_pool_hashrate() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  antpool
        let binding = format!("v1/mining/pool/antpool/hashrate").clone();
        let get_mining_pool_hashrate: &str = blocking(&binding).expect("returns current v1/mining/pool/:slug/hashrate");
        let get_mining_pool_hashrate = api("mining_pool_hashrate", "antpool", true);
        wait("1");

        ###  ...

        ###  foundryusa
        let binding = format!("v1/mining/pool/foundryusa/hashrate").clone();
        let get_mining_pool_hashrate: &str = blocking(&binding).expect("returns current v1/mining/pool/:slug/hashrate");
        let get_mining_pool_hashrate = api("mining_pool_hashrate", "foundryusa", true);
        wait("1");
    }
    #[test]
    fn test_mining_pool_blocks() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        use crate::args::mining_pool_blocks;
        ###  luxor
        let binding = format!("v1/mining/pool/luxor/blocks/730000").clone();
        let get_mining_pool_blocks: &str =
            blocking(&binding).expect("returns v1/mining/pool/:slug/blocks/[:blockHeight]");
        let get_mining_pool_blocks = mining_pool_blocks("luxor", "730000");
        wait("1");

        ###  ...

        ###  antpool
        let binding = format!("v1/mining/pool/antpool/blocks/730000").clone();
        let get_mining_pool_blocks: &str =
            blocking(&binding).expect("returns v1/mining/pool/:slug/blocks/[:blockHeight]");
        let get_mining_pool_blocks = mining_pool_blocks("antpool", "730000");
        wait("1");
    }
    #[test]
    fn test_mining_hashrate() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  1m
        let binding = format!("v1/mining/hashrate/1m").clone();
        let get_mining_hashrate: &str = blocking(&binding).expect("returns v1/mining/hashrate/[:timePeriod]");
        let get_mining_hashrate = api("mining_hashrate", "1m", true);
        wait("1");
        ###  3m
        let binding = format!("v1/mining/hashrate/3m").clone();
        let get_mining_hashrate: &str = blocking(&binding).expect("returns v1/mining/hashrate/[:timePeriod]");
        let get_mining_hashrate = api("mining_hashrate", "3m", true);
        wait("1");
        ###  6m
        let binding = format!("v1/mining/hashrate/6m").clone();
        let get_mining_hashrate: &str = blocking(&binding).expect("returns v1/mining/hashrate/[:timePeriod]");
        let get_mining_hashrate = api("mining_hashrate", "6m", true);
        wait("1");
        ###  1y
        let binding = format!("v1/mining/hashrate/1y").clone();
        let get_mining_hashrate: &str = blocking(&binding).expect("returns v1/mining/hashrate/[:timePeriod]");
        let get_mining_hashrate = api("mining_hashrate", "1y", true);
        wait("1");
        ###  2y
        let binding = format!("v1/mining/hashrate/2y").clone();
        let get_mining_hashrate: &str = blocking(&binding).expect("returns v1/mining/hashrate/[:timePeriod]");
        let get_mining_hashrate = api("mining_hashrate", "2y", true);
        wait("1");
        ###  3y
        let binding = format!("v1/mining/hashrate/3y").clone();
        let get_mining_hashrate: &str = blocking(&binding).expect("returns v1/mining/hashrate/[:timePeriod]");
        let get_mining_hashrate = api("mining_hashrate", "3y", true);
        wait("1");
        ###  all
        let binding = format!("v1/mining/hashrate/all").clone();
        let get_mining_hashrate: &str = blocking(&binding).expect("returns v1/mining/hashrate/[:timePeriod]");
        let get_mining_hashrate = api("mining_hashrate", "all", true);
        wait("1");
    }
    #[test]
    fn test_mining_difficulty_adjustments() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  1m
        let binding = format!("v1/mining/difficulty-adjustments/1m").clone();
        let get_mining_difficulty_adjustment: &str =
            blocking(&binding).expect("returns v1/mining/difficulty-adjustments/[:interval]");
        let get_mining_hashrate = api("mining_hashrate", "1m", true);
        wait("1");
        ###  3m
        let binding = format!("v1/mining/difficulty-adjustments/3m").clone();
        let get_mining_difficulty_adjustment: &str =
            blocking(&binding).expect("returns v1/mining/difficulty-adjustments/[:interval]");
        let get_mining_hashrate = api("mining_hashrate", "3m", true);
        wait("1");
        ###  6m
        let binding = format!("v1/mining/difficulty-adjustments/6m").clone();
        let get_mining_difficulty_adjustment: &str =
            blocking(&binding).expect("returns v1/mining/difficulty-adjustments/[:interval]");
        let get_mining_hashrate = api("mining_hashrate", "6m", true);
        wait("1");
        ###  1y
        let binding = format!("v1/mining/difficulty-adjustments/1y").clone();
        let get_mining_difficulty_adjustment: &str =
            blocking(&binding).expect("returns v1/mining/difficulty-adjustments/[:interval]");
        let get_mining_hashrate = api("mining_hashrate", "1y", true);
        wait("1");
        ###  2y
        let binding = format!("v1/mining/difficulty-adjustments/2y").clone();
        let get_mining_difficulty_adjustment: &str =
            blocking(&binding).expect("returns v1/mining/difficulty-adjustments/[:interval]");
        let get_mining_hashrate = api("mining_hashrate", "2y", true);
        wait("1");
        ###  3y
        let binding = format!("v1/mining/difficulty-adjustments/3y").clone();
        let get_mining_difficulty_adjustment: &str =
            blocking(&binding).expect("returns v1/mining/difficulty-adjustments/[:interval]");
        let get_mining_hashrate = api("mining_hashrate", "3y", true);
        wait("1");
        ###  all
        let binding = format!("v1/mining/difficulty-adjustments").clone();
        let get_mining_difficulty_adjustment: &str =
            blocking(&binding).expect("returns v1/mining/difficulty-adjustments/[:interval]");
        let get_mining_hashrate = api("mining_hashrate", "all", true);
        wait("1");
    }
    #[test]
    fn test_mining_reward_stats() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/v1/mining/reward-stats/:blockCount
        let blockCount = "100";
        let binding = format!("v1/mining/reward-stats/{blockCount}").clone();
        let mining_reward_stats: &str = blocking(&binding).expect("returns current v1/mining/reward-stats/:blockCount");
        let mining_reward_stats = api("mining_reward_stats", "100", true);
        wait("1");
    }

    ###  ...

    #[test]
    fn test_mining_blocks_audit_score() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/v1/mining/blocks/audit/score/:blockHash
        let blockHash = "000000000000000000032535698c5b0c48283b792cf86c1c6e36ff84464de785";
        let binding = format!("v1/mining/blocks/audit/score/{blockHash}").clone();
        let blocks_audit_score: &str =
            blocking(&binding).expect("returns current v1/mining/blocks/audit/score/:blockHash");
        let block_audit_score = api(
            "blocks_audit_score",
            "000000000000000000032535698c5b0c48283b792cf86c1c6e36ff84464de785",
            true,
        );
        wait("1");
    }
    #[test]
    fn test_mining_blocks_audit_scores() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/v1/mining/blocks/audit/scores/:startHeight
        let startHeight = "820000";
        let binding = format!("v1/mining/blocks/audit/scores/{startHeight}").clone();
        let blocks_audit_scores: &str =
            blocking(&binding).expect("returns current v1/mining/blocks/audit/scores/:startHeight");
        let blocks_audit_scores = api("blocks_audit_scores", "820000", true);
        wait("1");
    }
    #[test]
    fn test_mining_block_audit_summary() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/v1/block/:blockHash/audit-summary
        let blockHash = "00000000000000000000f218ceda7a5d9c289040b9c3f05ef9f7c2f4930e0123";
        let binding = format!("v1/block/{blockHash}/audit-summary").clone();
        let block_audit_summary: &str = blocking(&binding).expect("returns v1/block/{blockHash}/audit-summary");
        let block_audit_summary = api("block_audit_summary", "820000", true);
        wait("1");
    }

    ###  Fees
    #[test]
    fn test_fees_mempool_blocks() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/v1/fees/mempool-blocks
        let binding = format!("v1/fees/mempool-blocks").clone();
        let fees_mempool_blocks: &str = blocking(&binding).expect("returns v1/fees/mempool-blocks");
        let block_audit_summary = api("fees_mempool_blocks", &"", true);
        wait("1");
    }
    #[test]
    fn test_fees_recommended() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/v1/fees/recommended
        let binding = format!("v1/fees/recommended").clone();
        let fees_recommended: &str = blocking(&binding).expect("returns v1/fees/recommended");
        let block_audit_summary = api("fees_recommended", &"", true);
        wait("1");
    }

    ###  Mempool
    #[test]
    fn test_mempool() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/mempool
        let binding = format!("mempool").clone();
        let fees_recommended: &str = blocking(&binding).expect("returns mempool");
        let block_audit_summary = api("mempool", &"", true);
        wait("1");
    }

    ###  Transactions
    ###  CPFP
    ###  CHILDREN_PAY_FOR_PARENT
    #[test]
    fn test_children_pay_for_parent() {
        let reachable = reachable();
        assert_eq!(reachable, true);
        ###  GET /api/v1/cpfp
        let txid = "e09d8afb19968715a4492205b8db5fe41da144b0c1e4f7a756c8bf9742d4f1f4";
        let binding = format!("v1/cpfp/{txid}").clone();
        ###  AS OF BLOCK_HEIGHT
        let children_pay_for_parent: &str = blocking(&binding).expect("returns v1/cpfp/:txid");
        let children_pay_for_parent = api(
            "children_pay_for_parent",
            "e09d8afb19968715a4492205b8db5fe41da144b0c1e4f7a756c8bf9742d4f1f4",
            true,
        );
        wait("1");
    }

    ###  LIGHTNING TESTS
    ###  lighting_channels_from_node_pubkey
    #[test]
    fn test_lighting_channels_from_node_pubkey() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_channels_from_txid
    #[test]
    fn test_lighting_channels_from_txid() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_isp_nodes
    #[test]
    fn test_lighting_isp_nodes() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_network_status
    #[test]
    fn test_lighting_network_status() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_node_stats
    #[test]
    fn test_lighting_stats() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_node_stats_per_country
    #[test]
    fn test_lighting_stats_per_country() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_nodes_channels
    #[test]
    fn test_lighting_nodes_channels() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_nodes_in_country
    #[test]
    fn test_lighting_nodes_in_country() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_nodes_stats_per_isp
    #[test]
    fn test_lighting_nodes_stats_per_isp() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_top_nodes
    #[test]
    fn test_lighting_top_nodes() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_top_nodes_by_connectivity
    #[test]
    fn test_lighting_top_nodes_by_connectivity() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_top_nodes_by_liquidity
    #[test]
    fn test_lighting_top_nodes_by_liquidity() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }
    ###  lighting_top_oldests_nodes
    #[test]
    fn test_lighting_top_oldests_nodes() {
        let reachable = reachable();
        assert_eq!(reachable, true);
    }

    ###  Accelerator (Public)

    ###  Accelerator (Authenticated)

    #[test]
    fn test_add() {
         assert_eq!(add(1, 2), 3);
        wait("1");
    }

    #[test]
    fn test_bad_add() {
         This assert would fire and test will fail.
         Please note, that private functions can be tested too!
         assert_ne!(bad_add(1, 2), 3);
        wait("1");
    }
    use std::panic::{catch_unwind, AssertUnwindSafe};
    #[test]
    fn should_panic() {
        let msg = catch_unwind(AssertUnwindSafe(|| {
            panic!(" foo panic message");
        }));

        assert_ne!("foo panic message", *msg.unwrap_err().downcast_ref::<&str>().unwrap());
        wait("1");
    }
}
