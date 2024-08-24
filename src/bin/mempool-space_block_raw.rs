use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let block_hash = &args[1];
            let _res = blocking(&format!("/block/{}/raw", &block_hash));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
