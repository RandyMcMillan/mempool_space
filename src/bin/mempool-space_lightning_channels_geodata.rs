use mempool_space::api::blocking;
fn main() {
    {
        let _res = blocking(&"v1/lightning/channels-geo".to_string());
    }
}
