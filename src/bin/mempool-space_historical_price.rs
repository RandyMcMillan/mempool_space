use mempool_space::api::blocking;
use std::env;
use std::time::SystemTime;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let currency = &String::from(&args[1]);
        if args.len() > 1 {
            if args.len() < 3 {
                let timestamp = format!("{:}", format!("{:?}", SystemTime::now()));
                let _res = blocking(&format!(
                    "v1/historical-price?currency={}&timestamp={}",
                    &format!("{:}", &currency),
                    &format!("{:}", &timestamp)
                ));
            } else if args.len() == 3 {
                let timestamp = String::from(&args[2]);
                let _res = blocking(&format!(
                    "v1/historical-price?currency={}&timestamp={}",
                    &format!("{:}", &currency),
                    &format!("{:}", &timestamp)
                ));
            }
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
