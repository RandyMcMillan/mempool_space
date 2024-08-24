use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let ch = &args[1];
            let _res = blocking(&format!("v1/lightning/nodes/country/{}", &ch));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
