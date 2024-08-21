use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let mut address = &String::from("");
        if args.len() > 1 {
            address = &args[1];
        } else {
            // silence is golden
            std::process::exit(0);
        }
        let _res = blocking(&format!("v1/validate-address/{}", &address));
    }
}
