use mempool_space::api::blocking;
use std::env;
use std::time::SystemTime;

fn main() {
    let now_secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            let _res = blocking(&format!("v1/mining/blocks/timestamp/{:?}", now_secs.unwrap().as_secs()));
        } else if args.len() == 2 {
            let utc_secs = &args[1];
            let _res = blocking(&format!("v1/mining/blocks/timestamp/{}", &utc_secs));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
