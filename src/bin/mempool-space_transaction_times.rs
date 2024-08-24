use mempool_space::api::blocking;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        //v1/transaction-times?
        let mut query = String::from("v1/transaction-times?");
        let mut count = 0;
        for arg in args.clone() {
            if count > 0 {
                //print!("txId[]={}\n",arg);
                query.push_str(&("txId[]=".to_owned() + &arg));
                query.push_str(&("&".to_owned()));
            }
            count = count + 1;
        }
        //print!("{}", query);
        let _res = blocking(&format!("{}", &query));
    } else {
        // silence is golden
        std::process::exit(0);
    }
}
