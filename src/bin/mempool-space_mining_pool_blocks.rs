use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
        let mut slug = &String::from("");
            slug = &args[1];
            if args.len() > 2 {
                let blockheight = String::from("");
                blockheight = args[2].clone();
                let _res = blocking(format!("v1/mining/pool/{}/blocks/{}", &slug, &blockheight))
            } else {
                let mut blockheight = String::from("");
                blockheight = blocking(format!("/blocks/tip/height")).unwrap();
                //let _res = blocking(format!("v1/mining/pool/{}/blocks/{}", &slug, &blockheight))

            }
        } else {
            // silence is golden
            std::process::exit(0);
        }
        //print!("timestamp={}", timestamp);
        use std::time::SystemTime;
        let _res = blocking(&format!(
            "v1/historical-price?currency={}&timestamp={}",
            &format!("{:}", &currency),
            &format!("{:}", &timestamp)
        ));
    }
}
