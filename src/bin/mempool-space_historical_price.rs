use mempool_space::api::blocking;
use std::env;
use std::time::SystemTime;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let currency = &String::from(&args[1]);
        if args.len() > 1 {
            if args.len() < 3 {
                let timestamp = format!("{:?}", SystemTime::now()).to_string();
                let _res = blocking(&format!(
                    "v1/historical-price?currency={}&timestamp={}",
                    &(&currency).to_string(),
                    &timestamp.to_string()
                ));
            } else if args.len() == 3 {
                let timestamp = String::from(&args[2]);
                let _res = blocking(&format!(
                    "v1/historical-price?currency={}&timestamp={}",
                    &(&currency).to_string(),
                    &timestamp.to_string()
                ));
            }
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
