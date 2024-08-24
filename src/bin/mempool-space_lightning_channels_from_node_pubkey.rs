use mempool_space::api::blocking;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let public_key = &args[1];
        let _res = blocking(&format!(
            "v1/lightning/channels?public_key={}&status=active",
            &public_key
        ));
    } else if args.len() == 3 {
        let public_key = &args[1];
        let status = &args[2];
        let _res = blocking(&format!(
            "v1/lightning/channels?public_key={}&status={}",
            &public_key, &status
        ));
    } else {
        // silence is golden
        std::process::exit(0);
    }
}
