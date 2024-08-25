use mempool_space::api::blocking;

fn main() {
    {
        let _res = blocking(&"mempool/recent".to_string());
    }
}
