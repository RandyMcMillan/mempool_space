use mempool_space::api::blocking;

fn main() {
    {
        let _res = blocking(&"mempool".to_string());
    }
}
