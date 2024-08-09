use mempool_space::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let mut currency = &String::from("");
        let mut timestamp = String::from("");
        if args.len() > 1 {
            //currency first position
            currency = &args[1];
            if args.len() > 2 {
                //timestamp second position
                timestamp = args[2].clone();
            } else {
                //timestamp now in seconds
                //use std::time::SystemTime;
                //let now = format!("{:?}", SystemTime::now());
                timestamp = format!("{:}", format!("{:?}", SystemTime::now()));
            }
        } else {
            // silence is golden
            std::process::exit(0);
        }
        print!("timestamp={}", timestamp);
        use std::time::SystemTime;
        let _res = blocking(&format!(
            "v1/historical-price?currency={}&timestamp={}",
            &format!("{:}", &currency),
            &format!("{:}", &timestamp)
        ));
    }
}
