use mempool_space::api::blocking;

fn main() {
    let _res = blocking(&"mempool/txids".to_string());
}
