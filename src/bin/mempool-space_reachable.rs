use std::io::Read;
use std::time::{Instant, SystemTime};

use mempool_space::blocking::blocking;
use reqwest::Url;

// use ureq::get;

const URL: &str = "https://mempool.space/api/blocks/tip/height";

fn main() {
    let n = 1;
    {
        let start = Instant::now();
        let blocks_tip_height = String::from("/blocks/tip/height");
        let res = blocking(&blocks_tip_height);
        println!("blocking {:?} {:?} bytes", start.elapsed(), res);
    }
}
