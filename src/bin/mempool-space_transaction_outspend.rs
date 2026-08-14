use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 3 {
            let txid = &args[1];
            let index = &args[2];
            let _res = blocking(&format!("tx/{}/outspend/{}", &txid, &index));
        } else if args.len() == 2 {
            let txid = &args[1];
            let _res = blocking(&format!("tx/{}/outspend/0", &txid));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}

