use mempool_space::api::blocking;
use std::env;

fn main() {
    {
        let _res = blocking(&format!("mempool"));
    }
}
