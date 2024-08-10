use getopts::Options;
use std::env;
use std::io::IsTerminal;
use std::path::PathBuf;
use std::process;

use crate::blocking;

pub fn generic_sys_call(option: &str, sub_string: &str) {
    use std::process::Command;

    if sub_string == "v1" {
        print!("TODO: support --version v1 api versioning.")
    } else if sub_string == "v2" {
        print!("TODO: support --version v2 api versioning.")
    } else {
        let output = if cfg!(target_os = "windows") {
            Command::new(format!("mempool-space_{}", option))
                .args(["/C", sub_string])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new(format!("mempool-space_{}", option))
                .arg(sub_string)
                //.arg("echo hello")
                .output()
                .expect("failed to execute process")
        };

        let result = String::from_utf8(output.stdout)
            .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
            .unwrap();
        print!("{}", result);
    }
}

fn historical_prices(currency: &str, timestamp: &str) {
    //REF: mempool-space --historical_price --currency EUR --timestamp 150000000
    //EXPECT: {"prices":[{"time":1279497600,"EUR":0,"USD":0}],"exchangeRates":{"USDEUR":0.92,"USDGBP":0.78,"USDCAD":1.38,"USDCHF":0.87,"USDAUD":1.52,"USDJPY":146.79}}
    let _res = blocking(&format!(
        "v1/historical-price?currency={}&timestamp={}",
        &format!("{:}", &currency),
        &format!("{:}", &timestamp)
    ));
    // let _res = blocking(&format!(
    //     "v1/historical-price?currency={}&timestamp={}",
    //     &format!("{:}", "EUR"),
    //     &format!("{:}", "1500000000")
    // ));
}

/// Command-line arguments to parse.
#[derive(Debug, Default)]
pub struct Args {
    //REF: https://mempool.space/docs/api/rest

    // VERSION
    // pub version: Option<String>,
    // GENERAL
    /// v1/difficulty_adjustment
    pub difficulty_adjustment: Option<String>,
    /// v1/prices
    pub prices: Option<String>,
    /// v1/historical-price
    /// ENDPOINT
    /// GET /api/v1/historical-price?currency=EUR&timestamp=1500000000
    /// default USD
    pub currency: Option<String>,
    /// default now
    pub timestamp: Option<String>,
    pub historical_price: Option<String>,

    // ADDRESSES
    /// address.
    pub address: Option<String>,
    /// address_txs.
    pub address_txs: Option<String>,
    /// address_txs_chain.
    pub address_txs_chain: Option<String>,
    /// address_txs_mempool.
    pub address_txs_mempool: Option<String>,
    /// address_utxos.
    pub address_utxos: Option<String>,

    /// block.
    pub block: Option<String>,

    /// Configuration file.
    pub config: Option<PathBuf>,
    /// Server address.
    pub server: Option<String>,
    /// Authentication or delete token.
    pub auth: Option<String>,
    /// URL to shorten.
    pub url: Option<String>,
    /// Remote URL to download file.
    pub remote: Option<String>,
    /// Files to upload.
    pub files: Vec<String>,
    /// Whether if the file will disappear after being viewed once.
    pub oneshot: bool,
    /// Expiration time for the link.
    pub expire: Option<String>,
    /// Prettify the program output.
    pub prettify: bool,
    /// Whether if the server version should be printed.
    pub print_server_version: bool,
    /// List files on the server (file name, file size, expiry timestamp).
    pub list_files: bool,
    /// Delete files from server.
    pub delete: bool,
    /// Send filename header (give uploaded file a specific name).
    pub filename: Option<String>,
}

impl Args {
    /// Parses the command-line arguments.
    pub fn parse() -> Self {
        let mut opts = Options::new();

        //OPTFLAG
        opts.optflag("h", "help", "prints help information");
        opts.optflag("v", "vv", "prints version information");
        opts.optflag("V", "server-version", "retrieves the server version");
        opts.optflag("l", "list", "lists files on the server");
        opts.optflag("d", "delete", "delete files from server");
        opts.optflag("o", "oneshot", "generates one shot links");
        opts.optflag("p", "pretty", "prettifies the output");

        //mempool api intercepts
        // VERSION
        // premeptive support v1,v2 etc...
        // opts.optopt("", "version", "api call version path (v1/...)", "VERSION");
        // GENERAL
        opts.optflag("", "difficulty_adjustment", "difficulty_adjustment api call");
        opts.optflag("", "prices", "prices api call");
        opts.optflag("", "historical_price", "historical_price api call");
        opts.optopt("", "timestamp", "timestamp api call", "TIMESTAMP");
        opts.optopt("", "currency", "currency api call", "CURRENCY");

        // ADDRESSES
        opts.optopt("", "address", "address api call", "ADDRESS");
        opts.optopt("", "address_txs", "address_txs api call", "ADDRESS_TXS");
        opts.optopt("", "address_utxos", "address_utxos api call", "ADDRESS_UTXOS");

        opts.optopt("", "block", "block api call", "BLOCK");

        //OPTOPT
        opts.optopt("c", "config", "sets the configuration file", "CONFIG");
        opts.optopt("s", "server", "sets the address of the rustypaste server", "SERVER");
        opts.optopt("a", "auth", "sets the authentication or delete token", "TOKEN");
        opts.optopt("u", "url", "sets the URL to shorten", "URL");
        opts.optopt("r", "remote", "sets the remote URL for uploading", "URL");
        opts.optopt("e", "expire", "sets the expiration time for the link", "TIME");
        opts.optopt("n", "filename", "sets and overrides the filename", "NAME");

        let env_args: Vec<String> = env::args().collect();
        let matches = match opts.parse(&env_args[1..]) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Argument error: `{e}`");
                process::exit(1);
            }
        };

        //mempool api intercepts
        // VERSION
        // GENERAL
        if matches.opt_present("difficulty_adjustment") {
            generic_sys_call("difficulty_adjustment", &"v9999");
            std::process::exit(0);
        }
        if matches.opt_present("prices") {
            generic_sys_call("prices", &"v9999");
            std::process::exit(0);
        }
        if matches.opt_present("historical_price") {
            if matches.opt_present("currency") {
                //print!("currency={}\n", matches.opt_present("currency"));
                let currency = matches.opt_str("currency");
                //print!("currency={}", currency.clone().unwrap());
                if matches.opt_present("timestamp") {
                    //print!("timestamp={}\n", matches.opt_present("timestamp"));
                    let timestamp = matches.opt_str("timestamp");
                    historical_prices(&currency.as_ref().unwrap(), &timestamp.unwrap());
                } else {
                    historical_prices(&currency.unwrap(), "");
                }
            } else {
                historical_prices("", "");
            }

            //historical_prices(&"USD", &"1500000000");
            //historical_prices(&"EUR", &"1500000000");
            std::process::exit(0);
        }

        // ADDRESSES
        if matches.opt_present("address") {
            let address = matches.opt_str("address");
            //print!("86:address={}", address.unwrap());
            generic_sys_call("address", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("address_txs") {
            let address = matches.opt_str("address_txs");
            //print!("86:address-txs");
            //TODO support --address 1wiz18xYmhRX6xStj2b9t1rwWX4GKUgpv --address_txs <no arg>
            generic_sys_call("address_txs", &address.unwrap());
            std::process::exit(0);
        }
        if matches.opt_present("block") {
            print!("69:block");
            std::process::exit(0);
        }

        if matches.opt_present("h")
            || (matches.free.is_empty()
                && !matches.opt_present("u")
                && !matches.opt_present("r")
                && !matches.opt_present("V")
                && !matches.opt_present("l")
                && !matches.opt_present("d")
                && !matches.opt_present("v")
                && std::io::stdin().is_terminal())
        {
            let usage = format!(
                "\n{} {} \u{2014} {}.\
                \n\u{221F} written by {}\
                \n\u{221F} licensed under MIT <{}>\
                \n\nUsage:\n    {} [options] <file(s)>",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_AUTHORS"),
                env!("CARGO_PKG_REPOSITORY"),
                "mempool-space",
            );
            println!("{}", opts.usage(&usage));
            process::exit(0)
        }

        if matches.opt_present("v") {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            process::exit(0)
        }

        Args {
            config: env::var("RPASTE_CONFIG")
                .ok()
                .or_else(|| matches.opt_str("c"))
                .map(PathBuf::from),

            // mempool api intercepts
            // mempool api version
            // version: matches.opt_str("version"),

            // GENERAL
            difficulty_adjustment: matches.opt_str("difficulty_adjustment"),
            currency: matches.opt_str("currency"),
            prices: matches.opt_str("prices"),
            timestamp: matches.opt_str("timestamp"),
            historical_price: matches.opt_str("historical_price"),

            // ADDRESSES
            address: matches.opt_str("address"),
            address_txs: matches.opt_str("address_txs"),
            address_txs_chain: matches.opt_str("address_txs_chain"),
            address_txs_mempool: matches.opt_str("address_txs_mempool"),
            address_utxos: matches.opt_str("address_utxos"),
            block: matches.opt_str("block"),

            server: matches.opt_str("s"),
            auth: matches.opt_str("a"),
            url: matches.opt_str("u"),
            remote: matches.opt_str("r"),
            oneshot: matches.opt_present("o"),
            expire: matches.opt_str("e"),
            prettify: matches.opt_present("p"),
            print_server_version: matches.opt_present("V"),
            list_files: matches.opt_present("l"),
            delete: matches.opt_present("d"),
            filename: matches.opt_str("n"),
            files: matches.free,
        }
    }
}
