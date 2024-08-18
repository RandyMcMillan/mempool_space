use mempool_space::blocking::blocking;

fn main() {
    {
        let _res = blocking(&format!("/blocks/tip/hash"));
    }
}
