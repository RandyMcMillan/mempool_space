use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let slug = &args[1];
            let _res = blocking(&format!("v1/mining/pool/{}/blocks", &slug));
        } else if args.len() == 3 {
            let slug = &args[1];
            let blockheight = &args[2];
            let _res = blocking(&format!("v1/mining/pool/{}/blocks/{}", &slug, &blockheight));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
