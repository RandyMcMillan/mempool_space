use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 3 {
            let mut block = &String::from("");
            let mut index = &String::from("");
            block = &args[1];
            index = &args[2];
            let _res = blocking(&format!("block/{}/txid/{}", &block, &index));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
