use mempool_space::api::blocking;

fn main() {
    {
        let _res = blocking(&format!("/blocks/tip/height"));
    }
}
