use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let query = &args[1];
        let _res = blocking(&format!("v1/lightning/search?searchText={}", &query));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
