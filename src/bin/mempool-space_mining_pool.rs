use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let mut slug = &String::from("");
            slug = &args[1];
            let _res = blocking(&format!("v1/mining/pool/{}", &slug));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
