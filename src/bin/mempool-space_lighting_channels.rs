use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let mut channel_id = &String::from("");
            channel_id = &args[1];
            let _res = blocking(&format!("v1/lightning/channels/{}", &channel_id));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
