use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            //let mut interval = &String::from("");
            let interval = &args[1];
            let _res = blocking(&format!("v1/lightning/statistics/{}", &interval));
        } else if args.len() == 1 {
            let _res = blocking(&format!("v1/lightning/statistics/latest"));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
