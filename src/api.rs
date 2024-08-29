/// const API_VERSION: &str = "v1";
///
/// pub fn api(option: &str, sub_string: &str) -> String
pub fn api(option: &str, sub_string: &str) -> String {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new(format!("mempool-space_{}", option))
            .args(["/C", sub_string])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(format!("mempool-space_{}", option))
            .arg(sub_string)
            .output()
            .expect("failed to execute process")
    };

    let result = String::from_utf8(output.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("{}", result);
    result
}
use crate::URL;
/// pub fn blocking(api: &String) -> Result<&str, ascii::AsciiChar>
/// a formatted ureq::get(&call)
pub fn blocking(api: &String) -> Result<&str, ascii::AsciiChar> {
    //print!("api={}", api);
    let call = format!("{}/{}", URL, api);
    let mut body = ureq::get(&call)
        .call()
        .expect("calls to blocking(api: &String) needs to include /v1/<api_endpoint> in some cases.")
        .into_reader();
    let mut buf = Vec::new();
    body.read_to_end(&mut buf).unwrap();
    if !api.ends_with("raw") {
        //print!("!api.ends_with raw");
        let text = match std::str::from_utf8(&buf) {
            Ok(s) => s,
            Err(_) => panic!("Invalid ASCII data"),
        };
        print!("{}", text);
    } else {
        if api.ends_with("raw") {
            //print!("api.ends_with raw");
            print!("{:?}", &buf);
        }
        if api.ends_with("something_else") {
            //print!("api.ends_with something_else");
            print!("{:?}", &buf);
        }
    }
    Ok(api)
}
