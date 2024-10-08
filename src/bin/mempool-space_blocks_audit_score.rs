use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let block_hash = &args[1];
            let _res = blocking(&format!("v1/mining/blocks/audit/score/{}", &block_hash));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
