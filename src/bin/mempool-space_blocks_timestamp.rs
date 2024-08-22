use mempool_space::api::blocking;
use std::env;
use std::time::SystemTime;

fn main() {
    // match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    //     Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
    //     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    // };
    let now_secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            //let now_secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
            let _res = blocking(&format!("v1/mining/blocks/timestamp/{:?}", now_secs.unwrap().as_secs()));
        } else if args.len() == 2 {
            let mut utc_secs = &String::from("");
            utc_secs = &args[1];
            let _res = blocking(&format!("v1/mining/blocks/timestamp/{}", &utc_secs));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
