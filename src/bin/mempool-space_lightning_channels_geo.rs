use mempool_space::api::blocking;
use std::env;

/// [get-channel-geodata](https://mempool.space/docs/api/rest#get-channel-geodata-node)
fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let mut pubkey = &String::from("");
            pubkey = &args[1];
            let _res = blocking(&format!("v1/lightning/channels-geo/{}", &pubkey));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
