use reqwest::Url;
use std::io::Read;
use std::time::SystemTime;

/// pub fn blockheight() -> Result<f64, ascii::AsciiChar>
///
pub fn blockheight() -> Result<f64, ascii::AsciiChar> {
    //! inside 1
    //!
    //! inside 2
    //!
    //! inside 3
    //!
    //! [google.com](www.google.com)
    //!
    //! <https://google.com>
    //!
    //! - comment
    //!
    //!     - comment
    //!
    //! since_the_epoch
    let since_the_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("get millis error");
    let seconds = since_the_epoch.as_secs();
    let subsec_millis = since_the_epoch.subsec_millis() as u64;
    let _now_millis = seconds * 1000 + subsec_millis;
    let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
    let mut res = reqwest::blocking::get(url).unwrap();
    let mut tmp_string = String::new();
    res.read_to_string(&mut tmp_string).unwrap();
    let tmp_u64 = tmp_string.parse::<u64>().unwrap_or(0);
    let blockheight = tmp_u64 as f64;
    return Ok(blockheight);
}

/// pub fn function_example(){}
///
pub fn function_example() {

    //! inside comment
}
