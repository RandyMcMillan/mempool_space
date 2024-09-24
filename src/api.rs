/// mempool.space api
/// <https://mempool.space/api>
pub const URL: &[&str] = &[
    "https://mempool.space/api",
    "https://mempool.sweetsats.io/api",
    "http:://localhost", //TODO support local instance
];

#[allow(dead_code)]
/// mempool.space onion api
/// <http://mempoolhqx4isw62xs7abwphsq7ldayuidyx2v2oethdhhj6mlo2r6ad.onion/api>
pub const TOR_URL: &[&str] = &["http://mempoolhqx4isw62xs7abwphsq7ldayuidyx2v2oethdhhj6mlo2r6ad.onion/api"];

/// const API_VERSION: &str = "v1";
///
/// the print boolean is set to false in the mempool-space_dashboard
/// the print boolean is set to true when called from the command line utilities
///
/// pub fn api(option: &str, sub_string: &str, print: bool) -> String
pub fn api(option: &str, sub_string: &str, print: bool) -> String {
    let mut once: bool = false;
    let mut dashboard: std::process::Child;
    // detect if mempool-space --dashboard
    if option.contains("dashboard") {
        loop {
            if !once {
                info!("api({:}, {:})", option, sub_string);
                dashboard = if cfg!(target_os = "windows") {
                    std::process::Command::new(format!("mempool-space_{}", option))
                        .args(["/C", "--dashboard"])
                        .spawn()
                        .expect("failed to execute process")
                } else {
                    std::process::Command::new(format!("mempool-space_{}", option))
                        .arg("--dashboard")
                        .spawn()
                        .expect("failed to execute process")
                };

                let stdin = dashboard.stdin.take();
                let stdout = dashboard.stdout.take();
                let stderr = dashboard.stderr.take();
                let result = dashboard.wait();
                use log::info; //, warn};
                info!("{:?}", stdin);
                info!("{:?}", stdout);
                info!("{:?}", stderr);
                info!("{:?}", result);
                once = true;
            } else {
                std::process::exit(0);
            }
        }
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

    if print {
        print!("{}", result);
    }
    result
}
/// pub fn blocking(api: &String) -> Result<&str, ascii::AsciiChar>
/// prints to terminal
pub fn blocking(api: &String) -> Result<&str, ascii::AsciiChar> {
    if api.contains("dashboard") {
        print!("blocking:invoke dashboard {:?}", api);
        std::process::exit(0);
    }
    if api.contains("address") {
        //print!("api={:?}", api);
    }
    let call = format!("{}/{}", URL[0], api);
    let mut body = ureq::get(&call)
        .call()
        .expect("blocking(api: &String) GET {URL[0]}/{api} OR GET {URL[0]}/v1/{api}")
        .into_reader();
    let mut buf = Vec::new();
    body.read_to_end(&mut buf).unwrap();
    //let text = match std::str::from_utf8(&buf) {
    //    Ok(s) => s,
    //    Err(_) => panic!("Invalid ASCII data"),
    //};
    //print!("{}", text);
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
