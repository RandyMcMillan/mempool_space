use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            let _res = blocking(&"v1/mining/hashrate/all".to_string());
        } else if args.len() == 2 {
            let timeperiod = &args[1];
            let _res = blocking(&format!("v1/mining/hashrate/{}", &timeperiod));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
