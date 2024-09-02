/// pub const URL: &str = "https://mempool.space/api";
pub const URL: &str = "https://mempool.space/api";
#[allow(dead_code)]
/// pub const TOR_URL: &str = "http://mempoolhqx4isw62xs7abwphsq7ldayuidyx2v2oethdhhj6mlo2r6ad.onion/api";
pub const TOR_URL: &str = "http://mempoolhqx4isw62xs7abwphsq7ldayuidyx2v2oethdhhj6mlo2r6ad.onion/api";

/// const API_VERSION: &str = "v1";
///
/// pub fn api(option: &str, sub_string: &str) -> String
pub fn api(option: &str, sub_string: &str) -> String {
    if option.ends_with("dashboard") {
        print!("api:invoke dashboard {:}", option);
        let _ = if cfg!(target_os = "windows") {
            std::process::Command::new(format!("mempool-space_{}", option))
                //.args(["/C", sub_string])
                .spawn()
                .expect("failed to execute process")
        } else {
            std::process::Command::new(format!("mempool-space_{}", option))
                //.arg(sub_string)
                .spawn()
                .expect("failed to execute process")
        };
        //let result = String::from_utf8(output.stdout)
        //    .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        //    .unwrap();

        //return result;
        loop {}

        //std::process::exit(0);
    }

    let output = if cfg!(target_os = "windows") {
        std::process::Command::new(format!("mempool-space_{}", option))
            .args(["/C", sub_string])
            .output()
            .expect("failed to execute process")
    } else {
        std::process::Command::new(format!("mempool-space_{}", option))
            .arg(sub_string)
            .output()
            .expect("failed to execute process")
    };

    let result = String::from_utf8(output.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    result
}
/// pub fn blocking(api: &String) -> Result<&str, ascii::AsciiChar>
/// prints to terminal
pub fn blocking(api: &String) -> Result<&str, ascii::AsciiChar> {
    if api.ends_with("dashboard") {
        print!("blocking:invoke dashboard {:?}", api);
        std::process::exit(0);
    }
    let call = format!("{}/{}", URL, api);
    let mut body = ureq::get(&call)
        .call()
        .expect("blocking(api: &String) GET {URL}/{api} OR GET {URL}/v1/{api}")
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
        if api.ends_with("dashboard") {
            print!("invoke dashboard {:?}", &buf);
        }
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
