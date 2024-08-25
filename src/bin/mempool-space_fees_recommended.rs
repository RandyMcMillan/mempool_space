use mempool_space::api::blocking;
fn main() {
    let _res = blocking(&"v1/fees/recommended".to_string());
}
