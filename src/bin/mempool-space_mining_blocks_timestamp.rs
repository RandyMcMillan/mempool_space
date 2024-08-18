use std::io::Read;
use std::time::{Instant, SystemTime};

use mempool_space::blocking::blocking;
use reqwest::Url;

// use ureq::get;

const URL: &str = "https://mempool.space/api/v1/mining/blocks/timestamp/1672531200";
// const URL: &str = "https://mempool.space/api/blocks/tip";
// const URL: &str = "https://mempool.space/api/blocks/tip/height";

fn main() {
    let n = 1;
    {
        let start = Instant::now();
        let mining_blocks_timestamp = String::from("/mining/blocks/timestamp");
        let res = blocking(&mining_blocks_timestamp);
        println!("blocking {:?} {:?} bytes", start.elapsed(), res);
    }
}
