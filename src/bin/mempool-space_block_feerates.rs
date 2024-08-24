use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let timeperiod = &args[1];
            let _res = blocking(&format!("v1/mining/blocks/fee-rates/{}", &timeperiod));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
