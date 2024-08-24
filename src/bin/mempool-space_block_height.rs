use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let height = &args[1];
            let _res = blocking(&format!("/block-height/{}", &height));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
