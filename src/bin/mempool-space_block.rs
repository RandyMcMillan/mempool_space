use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let mut block = &String::from("");
            block = &args[1];
            let _res = blocking(&format!("/block/{}", &block));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
