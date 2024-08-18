const API_VERSION: &str = "v1";
///  `pub fn api(option: &str, sub_string: &str) -> String`
pub fn api(option: &str, sub_string: &str) -> String {
    use std::process::Command;

    //if sub_string == "v1" {
    //    print!("TODO: support --version v1 api versioning.");
    //} else if sub_string == "v2" {
    //    print!("TODO: support --version v2 api versioning.");
    //} else {
    let output = if cfg!(target_os = "windows") {
        Command::new(format!("mempool-space_{}", option))
            .args(["/C", sub_string])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new(format!("mempool-space_{}", option))
            .arg(sub_string)
            //.arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let result = String::from_utf8(output.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    print!("{}", result);
    result
    //}
}
