// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

//! Reachable, check if a Target is currently available or not.
//!
//! A "Target" is everything that implements the Target trait, used to
//! check if, a resource is currently available. This crate offers a ICMP and TCP based Target
//! usable to check, if a computer is available over the network.
//!
//! Additionally this crate contains asynchronous utilities to execute these checks regularly
//! within a given time interval.

use std::io::Read;

// use crate::blockheight::blockheight;

const VERSION: &str = "v1";
const URL: &str = "https://mempool.space/api";

pub fn blocking(api: &String) -> Result<&str, ()> {
    let call = format!("{}/{}", URL, api);
    // print!("{}", call);
    let mut body = ureq::get(&call).call().expect("REASON").into_reader();
    let mut buf = Vec::new();
    body.read_to_end(&mut buf).unwrap();
    // print block count from mempool.space or panic
    let text = match std::str::from_utf8(&buf) {
        Ok(s) => s,
        Err(_) => panic!("Invalid ASCII data"),
    };
    // let api: &str = text;
    print!("{}", text);
    // print!("{}", api);
    Ok(api)
    // buf.len()
}

// Modules
pub mod blockheight;
pub mod error;
pub mod resolve_policy;
pub mod target;

#[cfg(feature = "async")]
pub mod async_target;

// Re-exports
pub use error::{CheckTargetError, ParseTargetError, ResolveTargetError};
pub use resolve_policy::ResolvePolicy;
pub use target::{Fqhn, IcmpTarget, Port, Status, Target, TcpTarget};

#[cfg(feature = "async")]
pub use async_target::{AsyncTarget, AsyncTargetExecutor, BoxedHandler, BoxedTarget, OldStatus};
