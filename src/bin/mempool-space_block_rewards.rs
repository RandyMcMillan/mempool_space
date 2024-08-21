use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let mut interval = &String::from("");
            interval = &args[1];
            let _res = blocking(&format!("v1/mining/blocks/rewards/{}", &interval));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
