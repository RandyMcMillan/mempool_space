use mempool_space::blocking::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let mut timestamp = &String::from("");
        if args.len() > 1 {
            timestamp = &args[1];
        } else {
            // silence is golden
            std::process::exit(0);
        }
        let _res = blocking(&format!("v1/mining/blocks/timestamp/{}", &timestamp));
    }
}
