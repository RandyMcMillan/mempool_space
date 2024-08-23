use mempool_space::api::blocking;
use std::env;

/// use mempool_space::mempool_space_lightning_search::main
///
/// [GET /api/v1/lightning/nodes/:pubKey](https://mempool.space/api/v1/lightning/nodes/033ac2f9f7ff643c235cc247c521663924aff73b26b38118a6c6821460afcde1b3)
///
///	[get-lightning-node-stats](https://mempool.space/docs/api/rest#get-lightning-node-stats)
///
///	`curl -sSL "https://mempool.space/api/v1/lightning/nodes/033ac2f9f7ff643c235cc247c521663924aff73b26b38118a6c6821460afcde1b3"`
fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let pubkey = &args[1];
            let _res = blocking(&format!("v1/lightning/nodes/{}", &pubkey));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
