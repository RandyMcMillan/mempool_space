use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let block = &args[1];
            let _res = blocking(&format!("block/{}/txs", &block));
        } else if args.len() == 3 {
            let block = &args[1];
            let start_index = &args[2];

            let arg_2_mod_25_must_be_zero = args[2].parse::<usize>().unwrap() % 25;

            if arg_2_mod_25_must_be_zero == 0 {
                let _res = blocking(&format!("block/{}/txs/{}", &block, &start_index));
            }
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
