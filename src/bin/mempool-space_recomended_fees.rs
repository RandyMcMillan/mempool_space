use mempool_space::api::blocking;

fn main() {
    let _res = blocking(&format!("v1/fees/recommended"));
}
