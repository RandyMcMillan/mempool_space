use mempool_space::blocking::blocking;
use std::env;

fn main() {
    {
        let _res = blocking(&format!("v1/prices"));
    }
}
