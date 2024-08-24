use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let start_height = &args[1];
            let _res = blocking(&format!("v1/mining/blocks/audit/scores/{}", &start_height));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
