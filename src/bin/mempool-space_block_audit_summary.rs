use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let mut blockhash = &String::from("");
            blockhash = &args[1];
            let _res = blocking(&format!("v1/block/{}/audit-summary", &blockhash));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
