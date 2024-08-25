use std::time::Instant;

use mempool_space::api::{api, blocking};

// use ureq::get;
fn main() {
    {
        let start = Instant::now();
        let blocks_tip_height = String::from("blocks/tip/height");
        let res = blocking(&blocks_tip_height);
        println!("\nblocking {:?} {:?} bytes", start.elapsed(), res);
        let start = Instant::now();
        let blocks_tip_height = String::from("blocks_tip_height");
        let res = api(&blocks_tip_height, "");
        println!("\napi {:?} {:?} bytes", start.elapsed(), res);
    }
}
