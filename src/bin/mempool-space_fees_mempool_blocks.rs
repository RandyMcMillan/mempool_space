use mempool_space::api::blocking;
fn main() {
    let _res = blocking(&"v1/fees/mempool-blocks".to_string());
}
