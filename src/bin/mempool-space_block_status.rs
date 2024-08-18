use mempool_space::blocking::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let mut block_hash = &String::from("");
        if args.len() > 1 {
            block_hash = &args[1];
        } else {
            // silence is golden
            std::process::exit(0);
        }
        let _res = blocking(&format!("/block/{}/status", &block_hash));
    }
}
