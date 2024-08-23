use mempool_space::api::blocking;
use std::env;
use std::time::SystemTime;

// COMMAND EXAMPLE
// mempool-space_lighting_channels_from_txid c3173549f502ede6440d5c48ea74af5607d88484c7a912bbef73d430049f8af4 d78f0b41a263af3df91fa4171cc2f60c40196aaf8f4bde5d1c8ff4474cfe753b

/// [GET /api/v1/lightning/channels/txids?txId[]=:txid](https://mempool.space/api/v1/lightning/channels/txids?txId[]=c3173549f502ede6440d5c48ea74af5607d88484c7a912bbef73d430049f8af4&txId[]=d78f0b41a263af3df91fa4171cc2f60c40196aaf8f4bde5d1c8ff4474cfe753b)
///
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        //v1/lightning/channels/txids?
        let mut query = String::from("v1/lightning/channels/txids?");
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
