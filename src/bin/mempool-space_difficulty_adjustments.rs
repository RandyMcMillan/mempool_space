use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let interval = &args[1];
            let _res = blocking(&format!("v1/mining/difficulty-adjustments/{}", &interval));
        } else if args.len() == 1 {
            let _res = blocking(&"v1/mining/difficulty-adjustments".to_string());
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
