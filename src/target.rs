// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

//! Module containing "Target" related functionality.

// Imports
use super::{CheckTargetError, ParseTargetError, ResolvePolicy};
use std::convert::From;
use std::fmt::{self};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, TcpStream};
use std::num::ParseIntError;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::Duration;

// Test imports
#[cfg(test)]
use mockall::automock;

/// Default timeout duration for each connection attempt of a [TcpTarget]
pub const DEFAULT_TCP_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// Alias of String expressing a "fully qualified domain name"
pub type Fqhn = String;

/// Alias of u16 expressing a port number
pub type Port = u16;

/// Trait specifying a Target that can be used to check if its available.
#[cfg_attr(test, automock)]
pub trait Target {
    /// Get a Targets identifier.
    ///
    /// # Returns
    /// A unique identifier of this Target.
    ///
    /// # Example
    /// ```
    /// # use std::str::FromStr;
    /// # use mempool_space::{Target, IcmpTarget};
    ///
    /// assert_eq!(IcmpTarget::from_str("127.0.0.1").unwrap().get_id(), "127.0.0.1");
    /// ```
    fn get_id(&self) -> String;

    /// Check if a Target is currently available.
    ///
    /// # Returns
    /// * On success, the current [Status] of this [Target].
    /// * On failure, a [CheckTargetError]. This error should be returned in case some internal error occurred.
    ///
    /// # Notes
    /// This method should be implemented in a non-blocking way to improve performance then used
    /// from an async execution context.
    ///
    /// # Example
    /// ```
    /// # use std::str::FromStr;
    /// # use mempool_space::{Status, Target, IcmpTarget};
    ///
    /// assert_eq!(
    ///     IcmpTarget::from_str("127.0.0.1").unwrap().check_availability().unwrap(),
    ///     Status::Available
    /// );
    /// ```
    fn check_availability(&self) -> Result<Status, CheckTargetError>;
}

/// Current status of a [Target]
#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    /// The state of a [Target] is unknown.
    Unknown,
    /// A [Target] is available
    Available,
    /// A [Target] is not available
    NotAvailable,
}

impl fmt::Display for Status {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Unknown => write!(formatter, "unknown"),
            Status::Available => write!(formatter, "available"),
            Status::NotAvailable => write!(formatter, "not available"),
        }
    }
}

/// Target to check if a system can be reached via ICMP.
///
/// # Notes
/// IcmpTargets use the ping command to perform availability checks.
/// Some administrator blackhole ICMP packets, leading to systems that look unavailable
/// although they can be reached with a [TcpTarget].
#[derive(Debug)]
pub struct IcmpTarget {
    /// [Fqhn] specifying a system to connect to.
    fqhn: Fqhn,
    /// [ResolvePolicy] to apply during resolution of fqhn to IP addresses.
    resolve_policy: ResolvePolicy,
}

impl IcmpTarget {
    /// Construct an [IcmpTarget].
    ///
    /// # Arguments
    /// * fqhn: string containing "fully qualified domain name" e.g. "::1", "localhost".
    /// * resolve_policy: the [ResolvePolicy] to use for this [Target].
    ///
    /// # Returns
    /// Instance of [IcmpTarget].
    ///
    /// # Notes
    /// For more convenience use the implementations of trait "From" and "FromStr".
    pub fn new(fqhn: Fqhn, resolve_policy: ResolvePolicy) -> Self {
        IcmpTarget { fqhn, resolve_policy }
    }

    /// Set a new [ResolvePolicy] for name resolution.
    pub fn set_resolve_policy(mut self, resolve_policy: ResolvePolicy) -> Self {
        self.resolve_policy = resolve_policy;
        self
    }

    /// Get a reference to the [Fqhn].
    pub fn get_fqhn(&self) -> &Fqhn {
        &self.fqhn
    }

    /// Get a reference to the [ResolvePolicy] in use.
    pub fn get_resolve_policy(&self) -> &ResolvePolicy {
        &self.resolve_policy
    }
}

impl Target for IcmpTarget {
    fn get_id(&self) -> String {
        String::from(self.get_fqhn())
    }

    fn check_availability(&self) -> Result<Status, CheckTargetError> {
        // Note: Spawn Ping to check if an ICMP target is available.
        // Using ping seems to be the easiest way to send ICMP packets without root privileges
        let available_via_ping = |addr: IpAddr| {
            if addr.is_ipv6() {
                Command::new("ping")
                    .stdout(Stdio::null())
                    .args(["-c", "1"])
                    .arg("-6")
                    .arg(addr.to_string())
                    .status()
                    .unwrap()
                    .success()
            } else {
                Command::new("ping")
                    .stdout(Stdio::null())
                    .args(["-c", "1"])
                    .arg(addr.to_string())
                    .status()
                    .unwrap()
                    .success()
            }
        };

        let addrs = self.resolve_policy.resolve(&self.fqhn)?;
        if addrs.into_iter().any(available_via_ping) {
            Ok(Status::Available)
        } else {
            Ok(Status::NotAvailable)
        }
    }
}

impl From<IpAddr> for IcmpTarget {
    fn from(addr: IpAddr) -> Self {
        IcmpTarget::new(addr.to_string(), ResolvePolicy::Agnostic)
    }
}

impl From<Ipv4Addr> for IcmpTarget {
    fn from(addr: Ipv4Addr) -> Self {
        IcmpTarget::new(addr.to_string(), ResolvePolicy::ResolveToIPv4)
    }
}

impl From<Ipv6Addr> for IcmpTarget {
    fn from(addr: Ipv6Addr) -> Self {
        IcmpTarget::new(addr.to_string(), ResolvePolicy::ResolveToIPv6)
    }
}

impl FromStr for IcmpTarget {
    type Err = ParseTargetError;

    fn from_str(s: &str) -> Result<IcmpTarget, Self::Err> {
        if s.is_empty() {
            Err(ParseTargetError::from("No FQHN found"))
        } else {
            Ok(IcmpTarget::new(String::from(s), ResolvePolicy::Agnostic))
        }
    }
}

/// Target to check if a system can be reached via TCP.
///
/// # Notes
/// TcpTargets use the blocking [TcpStream::connect_timeout] method to establish a
/// connection. Depending on the remote system, this can take a while. If TcpTargets
/// are used in an async context, try to speedup [Target::check_availability] by configuring a shorter
/// connect_timeout.
///
/// TcpTargets on check_availability() to open a connection to the remote target and close
/// it afterwards. This means that the service behind the target port, must be able to
/// handle spontaneous connection closing.
#[derive(Debug)]
pub struct TcpTarget {
    /// [Fqhn] specifying a system to connect to.
    fqhn: Fqhn,
    /// [Port] specifying the TCP port to connect to.
    port: Port,
    /// [Duration] used as connect_timeout
    connect_timeout: Duration,
    /// [ResolvePolicy] to apply during resolution of fqhn to IP addresses.
    resolve_policy: ResolvePolicy,
}

impl TcpTarget {
    /// Construct an [TcpTarget].
    ///
    /// # Arguments
    /// * fqhn: string containing "fully qualified domain name" e.g. "::1", "localhost".
    /// * port: port number to connect to.
    /// * connect_timeout: [Duration] used as connection attempt timeout.
    /// * resolve_policy: the [ResolvePolicy] to use for this [Target].
    ///
    /// # Returns
    /// Instance of [TcpTarget].
    ///
    /// # Notes
    /// For more convenience use the implementations of trait "From" and "FromStr".
    pub fn new(fqhn: Fqhn, port: Port, connect_timeout: Duration, resolve_policy: ResolvePolicy) -> Self {
        TcpTarget {
            fqhn,
            port,
            connect_timeout,
            resolve_policy,
        }
    }

    /// Set a new [ResolvePolicy] for name resolution.
    pub fn set_resolve_policy(mut self, resolve_policy: ResolvePolicy) -> Self {
        self.resolve_policy = resolve_policy;
        self
    }

    /// Set a new connect_timeout [Duration] for [TcpStream::connect_timeout]
    /// attempts used in [Target::check_availability].
    pub fn set_connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = connect_timeout;
        self
    }

    /// Get a reference to the [Fqhn].
    pub fn get_fqhn(&self) -> &Fqhn {
        &self.fqhn
    }

    /// Get a reference to the TCP [Port] number in use.
    pub fn get_portnumber(&self) -> &Port {
        &self.port
    }

    /// Get a reference to the connect_timeout [Duration] in use.
    pub fn get_connect_timeout(&self) -> &Duration {
        &self.connect_timeout
    }

    /// Get a reference to the [ResolvePolicy] in use.
    pub fn get_resolve_policy(&self) -> &ResolvePolicy {
        &self.resolve_policy
    }
}

impl Target for TcpTarget {
    fn get_id(&self) -> String {
        format!("{}:{}", self.get_fqhn(), self.get_portnumber())
    }

    fn check_availability(&self) -> Result<Status, CheckTargetError> {
        // Check TCP availability: Try to establish a connection with the given Target.
        // If the connection was established, tear it down immediately. All standard
        // Network services should be able to deal with this behavior.

        // Resolve and construct address/port pairs
        // Try for each address/port pair to establish a connection.
        // Occurring errors are treated as a sign of target is not available.
        let addrs = self.resolve_policy.resolve(&self.fqhn)?;
        let available = addrs
            .into_iter()
            .map(|addr| SocketAddr::from((addr, self.port)))
            .any(|addr| TcpStream::connect_timeout(&addr, self.connect_timeout).is_ok());

        if available {
            Ok(Status::Available)
        } else {
            Ok(Status::NotAvailable)
        }
    }
}

impl From<SocketAddr> for TcpTarget {
    fn from(socket: SocketAddr) -> Self {
        TcpTarget::new(
            socket.ip().to_string(),
            socket.port(),
            DEFAULT_TCP_CONNECT_TIMEOUT,
            ResolvePolicy::Agnostic,
        )
    }
}

impl From<SocketAddrV4> for TcpTarget {
    fn from(socket: SocketAddrV4) -> Self {
        TcpTarget::new(
            socket.ip().to_string(),
            socket.port(),
            DEFAULT_TCP_CONNECT_TIMEOUT,
            ResolvePolicy::ResolveToIPv4,
        )
    }
}

impl From<SocketAddrV6> for TcpTarget {
    fn from(socket: SocketAddrV6) -> Self {
        TcpTarget::new(
            socket.ip().to_string(),
            socket.port(),
            DEFAULT_TCP_CONNECT_TIMEOUT,
            ResolvePolicy::ResolveToIPv6,
        )
    }
}

impl From<(IpAddr, u16)> for TcpTarget {
    fn from(pieces: (IpAddr, u16)) -> Self {
        TcpTarget::from(SocketAddr::from(pieces))
    }
}

impl From<(Ipv4Addr, u16)> for TcpTarget {
    fn from(pieces: (Ipv4Addr, u16)) -> Self {
        let (addr, port) = pieces;
        TcpTarget::from(SocketAddrV4::new(addr, port))
    }
}

impl From<(Ipv6Addr, u16)> for TcpTarget {
    fn from(pieces: (Ipv6Addr, u16)) -> Self {
        let (addr, port) = pieces;
        TcpTarget::from(SocketAddrV6::new(addr, port, 0, 0))
    }
}

impl FromStr for TcpTarget {
    type Err = ParseTargetError;

    fn from_str(s: &str) -> Result<TcpTarget, Self::Err> {
        if let Some(index) = s.rfind(':') {
            // Extract and verify FQHN
            let fqhn = String::from(&s[..index]);
            if fqhn.is_empty() {
                return Err(ParseTargetError::from("No FQHN found"));
            }

            // Extract and verify Portnumber
            let maybe_port = &s[index + 1..];
            match maybe_port.parse() as Result<u16, ParseIntError> {
                Ok(port) => {
                    if port == 0 {
                        Err(ParseTargetError::from("Invalid Portnumber '0' found"))
                    } else {
                        Ok(TcpTarget::new(
                            fqhn,
                            port,
                            DEFAULT_TCP_CONNECT_TIMEOUT,
                            ResolvePolicy::Agnostic,
                        ))
                    }
                }
                Err(err) => Err(ParseTargetError::from(("Failed to parse Portnumber", err))),
            }
        } else {
            Err(ParseTargetError::from("Missing ':' between host and port"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    use super::*;

    // IcmpTarget tests
    #[test]
    fn icmp_target_from() {
        // Expectency: The IcmpTarget offer multiple conversion implementations.
        // This test has to ensure that they are working correctly.
        // 1) from<IpAddr>
        let target = IcmpTarget::from(IpAddr::V4(Ipv4Addr::LOCALHOST));
        assert_eq!(target.fqhn, String::from("127.0.0.1"));
        assert_eq!(target.resolve_policy, ResolvePolicy::Agnostic);

        // 2) from<Ipv4Addr>
        let target = IcmpTarget::from(Ipv4Addr::LOCALHOST);
        assert_eq!(target.fqhn, String::from("127.0.0.1"));
        assert_eq!(target.resolve_policy, ResolvePolicy::ResolveToIPv4);

        // 3) from<Ipv6Addr>
        let target = IcmpTarget::from(Ipv6Addr::LOCALHOST);
        assert_eq!(target.fqhn, String::from("::1"));
        assert_eq!(target.resolve_policy, ResolvePolicy::ResolveToIPv6);
    }

    #[test]
    fn icmp_target_from_str_valid() {
        // Expectency: The IcmpTarget offer multiple conversion implementations.
        // This test has to ensure that they are working correctly.
        let target = IcmpTarget::from_str("127.0.0.1").unwrap();
        assert_eq!(target.fqhn, "127.0.0.1");
        assert_eq!(target.resolve_policy, ResolvePolicy::Agnostic);
    }

    #[test]
    fn icmp_target_from_str_invalid() {
        // Expectency: The IcmpTarget returns an error if fqhn is an empty string.
        assert_eq!(format!("{}", IcmpTarget::from_str("").unwrap_err()), "No FQHN found");
    }

    #[test]
    fn icmp_target_get_id() {
        // Expectency: get_id must return the FQHN for ICMP targets
        assert_eq!(IcmpTarget::from_str("www.google.de").unwrap().get_id(), "www.google.de");
        assert_eq!(IcmpTarget::from(Ipv4Addr::LOCALHOST).get_id(), "127.0.0.1");
    }

    #[test]
    fn icmp_target_check_availability() {
        // Expectency: LOCALHOST must always be available without any errors
        let target = IcmpTarget::from(Ipv4Addr::LOCALHOST);
        let status = target.check_availability().unwrap();
        assert_eq!(status, Status::Available);
    }

    #[test]
    #[ignore]
    fn icmp_target_check_availability_invalid_host_error() {
        // Expectency: A invalid host must lead to an error
        let target = IcmpTarget::from_str("asdkjhasjdkhakjsdhsad").unwrap();
        let status = target.check_availability();
        assert_eq!(
            format!("{}", status.unwrap_err()),
            "ResolveTargetError caused by: IoError caused by: failed to lookup \
             address information: Name or service not known"
        );
    }

    #[test]
    fn icmp_target_check_availability_all_addresses_filtered_error_v4() {
        // Expectency: check_availability must return an error if all resolved
        //             IPv4 addresses were discarded by the ResolvePolicy
        let target = IcmpTarget::from(Ipv4Addr::LOCALHOST);
        let target = target.set_resolve_policy(ResolvePolicy::ResolveToIPv6);
        let status = target.check_availability();
        assert_eq!(
            format!("{}", status.unwrap_err()),
            "ResolveTargetError caused by: Given Policy filtered all resolved addresses"
        );
    }

    #[test]
    fn icmp_target_check_availability_all_addresses_filtered_error_v6() {
        // Expectency: check_availability must return an error if all resolved
        //             IPv6 addresses were discarded by the ResolvePolicy
        let target = IcmpTarget::from(Ipv6Addr::LOCALHOST);
        let target = target.set_resolve_policy(ResolvePolicy::ResolveToIPv4);
        let status = target.check_availability();
        assert_eq!(
            format!("{}", status.unwrap_err()),
            "ResolveTargetError caused by: Given Policy filtered all resolved addresses"
        );
    }

    // TcpTarget tests
    #[test]
    fn tcp_target_from() {
        // Expectency: The TcpTarget offer multiple conversion implementations.
        // This test has to ensure that they are working correctly.
        let expected_port = 1024;

        // 1) from<SocketAddr>
        let target = TcpTarget::from(SocketAddr::from((Ipv4Addr::LOCALHOST, expected_port)));
        assert_eq!(target.fqhn, "127.0.0.1");
        assert_eq!(target.port, expected_port);
        assert_eq!(target.resolve_policy, ResolvePolicy::Agnostic);

        // 2) from<SocketAddrV4>
        let target = TcpTarget::from(SocketAddrV4::new(Ipv4Addr::LOCALHOST, expected_port));
        assert_eq!(target.fqhn, "127.0.0.1");
        assert_eq!(target.port, expected_port);
        assert_eq!(target.resolve_policy, ResolvePolicy::ResolveToIPv4);

        // 3) from<SocketAddrV6>
        let target = TcpTarget::from(SocketAddrV6::new(Ipv6Addr::LOCALHOST, expected_port, 0, 0));
        assert_eq!(target.fqhn, "::1");
        assert_eq!(target.port, expected_port);
        assert_eq!(target.resolve_policy, ResolvePolicy::ResolveToIPv6);

        // 5) from<IpAddr>
        let target = TcpTarget::from((IpAddr::V4(Ipv4Addr::LOCALHOST), expected_port));
        assert_eq!(target.fqhn, "127.0.0.1");
        assert_eq!(target.port, expected_port);
        assert_eq!(target.resolve_policy, ResolvePolicy::Agnostic);

        // 5) from<Ipv4Addr>
        let target = TcpTarget::from((Ipv4Addr::LOCALHOST, expected_port));
        assert_eq!(target.fqhn, "127.0.0.1");
        assert_eq!(target.port, expected_port);
        assert_eq!(target.resolve_policy, ResolvePolicy::ResolveToIPv4);

        // 6) from<Ipv6Addr>
        let target = TcpTarget::from((Ipv6Addr::LOCALHOST, expected_port));
        assert_eq!(target.fqhn, "::1");
        assert_eq!(target.port, expected_port);
        assert_eq!(target.resolve_policy, ResolvePolicy::ResolveToIPv6);
    }

    #[test]
    fn tcp_target_from_str_valid() {
        // Expectency: The TcpTarget offer multiple conversion implementations.
        // This test has to ensure that they are working correctly.

        // from_str with valid IPv4 Address and port
        let target = TcpTarget::from_str("127.0.0.1:1024").unwrap();
        assert_eq!(target.fqhn, "127.0.0.1");
        assert_eq!(target.port, 1024);
        assert_eq!(target.resolve_policy, ResolvePolicy::Agnostic);

        // from_str with valid IPv6 Address and port
        let target = TcpTarget::from_str("[::1]:1024").unwrap();
        assert_eq!(target.fqhn, "[::1]");
        assert_eq!(target.port, 1024);
        assert_eq!(target.resolve_policy, ResolvePolicy::Agnostic);
    }

    #[test]
    fn tcp_target_from_str_invalid_no_double_colon() {
        // Expectency: The TcpTarget returns an error if string contains no :.
        assert_eq!(
            format!("{}", TcpTarget::from_str("1024").unwrap_err()),
            "Missing ':' between host and port"
        );
    }

    #[test]
    fn tcp_target_from_str_invalid_no_port() {
        // Expectency: The TcpTarget returns an error if string contains no port.
        assert_eq!(
            format!("{}", TcpTarget::from_str("foo:").unwrap_err()),
            "Failed to parse Portnumber caused by: cannot parse integer from empty string"
        );
    }

    #[test]
    fn tcp_target_from_str_invalid_port() {
        // Expectency: The TcpTarget returns an error if string contains no port number.
        assert_eq!(
            format!("{}", TcpTarget::from_str("foo:12bar32").unwrap_err()),
            "Failed to parse Portnumber caused by: invalid digit found in string"
        );
    }

    #[test]
    fn tcp_target_from_str_invalid_port_overflow() {
        // Expectency: The TcpTarget returns an error if portnumber overflows u16.
        assert_eq!(
            format!("{}", TcpTarget::from_str("foo:65536").unwrap_err()),
            "Failed to parse Portnumber caused by: number too large to fit in target type"
        );
    }

    #[test]
    fn tcp_target_from_str_invalid_port_zero() {
        // Expectency: The TcpTarget returns an error if portnumber is 0 (invalid port).
        assert_eq!(
            format!("{}", TcpTarget::from_str("foo:0").unwrap_err()),
            "Invalid Portnumber '0' found"
        );
    }

    #[test]
    fn tcp_target_from_str_invalid_no_fqhn() {
        // Expectency: The TcpTarget returns an error if fqhn is an empty string.
        assert_eq!(
            format!("{}", TcpTarget::from_str(":1024").unwrap_err()),
            "No FQHN found"
        );
    }

    #[test]
    fn tcp_target_get_id() {
        // Expectency: get_id must return the FQHN + Portnumber for TCP targets
        assert_eq!(
            TcpTarget::from_str("www.google.de:1024").unwrap().get_id(),
            "www.google.de:1024"
        );
        assert_eq!(TcpTarget::from((Ipv4Addr::LOCALHOST, 23)).get_id(), "127.0.0.1:23");
    }

    #[test]
    fn tcp_target_check_availability() {
        // Expectency: check_availability must return Status::Available if a peer accepts a
        //             connection.
        let srv = spawn(|| TcpListener::bind("127.0.0.1:24211").unwrap().accept().unwrap());
        sleep(Duration::from_millis(500));

        // Connect to local TCP connection
        let target = TcpTarget::from_str("127.0.0.1:24211").unwrap();
        let status = target.check_availability().unwrap();
        assert_eq!(status, Status::Available);

        // Join spawned thread
        srv.join().unwrap();
    }

    #[test]
    fn tcp_target_check_unavailability() {
        // Expectency: check_availability must return Status::NotAvailable if on a closed port.
        // Connect to local TCP connection
        let target = TcpTarget::from_str("127.0.0.1:24212").unwrap();
        let status = target.check_availability().unwrap();
        assert_eq!(status, Status::NotAvailable);
    }

    #[test]
    #[ignore]
    fn tcp_target_check_availability_invalid_host_error() {
        // Expectency: A invalid host must lead to an error
        let target = TcpTarget::from_str("asdkjhasjdkhakjsdhsad:1025").unwrap();
        let status = target.check_availability();
        assert_eq!(
            format!("{}", status.unwrap_err()),
            "ResolveTargetError caused by: IoError caused by: failed to lookup address information: Name or service not known"
        );
    }

    #[test]
    fn tcp_target_check_availability_all_addresses_filtered_error_v4() {
        // Expectency: check_availability must return an error if all resolved
        //             IPv4 addresses were discarded by the ResolvePolicy
        let target = TcpTarget::from((Ipv4Addr::LOCALHOST, 1024)).set_resolve_policy(ResolvePolicy::ResolveToIPv6);
        let status = target.check_availability();
        assert_eq!(
            format!("{}", status.unwrap_err()),
            "ResolveTargetError caused by: Given Policy filtered all resolved addresses"
        );
    }

    #[test]
    fn tcp_target_check_availability_all_addresses_filtered_error_v6() {
        // Expectency: check_availability must return an error if all resolved
        //             IPv6 addresses were discarded by the ResolvePolicy
        let target = TcpTarget::from((Ipv6Addr::LOCALHOST, 1024)).set_resolve_policy(ResolvePolicy::ResolveToIPv4);
        let status = target.check_availability();
        assert_eq!(
            format!("{}", status.unwrap_err()),
            "ResolveTargetError caused by: Given Policy filtered all resolved addresses"
        );
    }
}
