use mempool_space::api::blocking;
use std::env;

/// curl -X POST -sSLd "0200000001fd5b5fcd1cb066c27cfc9fda5428b9be850b81ac440ea51f1ddba2f987189ac1010000008a4730440220686a40e9d2dbffeab4ca1ff66341d06a17806767f12a1fc4f55740a7af24c6b5022049dd3c9a85ac6c51fecd5f4baff7782a518781bbdd94453c8383755e24ba755c01410436d554adf4a3eb03a317c77aa4020a7bba62999df633bba0ea8f83f48b9e01b0861d3b3c796840f982ee6b14c3c4b7ad04fcfcc3774f81bff9aaf52a15751fedfdffffff02416c00000000000017a914bc791b2afdfe1e1b5650864a9297b20d74c61f4787d71d0000000000001976a9140a59837ccd4df25adc31cdad39be6a8d97557ed688ac00000000" "https://mempool.space/api/tx"

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            let tx = &args[1];
            // TODO
            let _res = blocking(&format!("tx {}", &tx));
        } else {
            // silence is golden
            std::process::exit(0);
        }
    }
}
