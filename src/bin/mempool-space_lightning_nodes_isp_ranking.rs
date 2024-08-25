use mempool_space::api::blocking;

fn main() {
    let _res = blocking(&"v1/lightning/nodes/isp-ranking".to_string());
}
