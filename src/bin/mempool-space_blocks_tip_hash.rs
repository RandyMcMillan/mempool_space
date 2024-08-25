use mempool_space::api::blocking;

fn main() {
    {
        let _res = blocking(&"/blocks/tip/hash".to_string());
    }
}
