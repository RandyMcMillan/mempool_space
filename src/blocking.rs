use crate::URL;
///  `pub fn blocking(api: &String) -> Result<&str>`
///
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
