use mempool_space::api::blocking;

fn main() {
    let _res = blocking(&"v1/fullrbf/replacements".to_string());
}
