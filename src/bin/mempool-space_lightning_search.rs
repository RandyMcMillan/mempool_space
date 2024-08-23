use mempool_space::api::blocking;
use std::env;

/// use mempool_space::mempool_space_lightning_search::main
///
/// [GET /api/v1/lightning/search?searchText=:query](https://mempool.space/api/v1/lightning/search?searchText=ACINQ)
///
///	[get-lightning-nodes-channels](https://mempool.space/docs/api/rest#get-lightning-nodes-channels)
///
///	`curl -sSL "https://mempool.space/api/v1/lightning/search?searchText=ACINQ"`
fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let query = &args[1];
            let _res = blocking(&format!("v1/lightning/search?searchText={}", &query));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
