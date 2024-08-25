use mempool_space::api::blocking;

fn main() {
    {
        let _res = blocking(&"v1/difficulty-adjustment".to_string());
    }
}
