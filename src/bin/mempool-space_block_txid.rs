use mempool_space::blocking::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let mut block = &String::from("");
        let mut index = &String::from("");
        if args.len() == 3 {
            block = &args[1];
            index = &args[2];
        } else {
            // silence is golden
            std::process::exit(0);
        }
        let _res = blocking(&format!("block/{}/txid/{}", &block, &index));
    }
}
