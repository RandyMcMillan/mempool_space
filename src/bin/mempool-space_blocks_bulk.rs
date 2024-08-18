use mempool_space::blocking::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let min_height = &args[1];
            let _res = blocking(&format!("v1/blocks-bulk/{}", &min_height));
        } else if args.len() == 3 {
            let min_height = &args[1];
            let max_height = &args[2];
            let _res = blocking(&format!("v1/blocks-bulk/{}/{}", &min_height, &max_height));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
