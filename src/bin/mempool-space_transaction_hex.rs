use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let txid = &args[1];
            let _res = blocking(&format!("tx/{}/hex", &txid));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
