// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

//! Module containing everything related network name resolution and filtering
//! of the resolved IP addresses.

// Imports
use super::ResolveTargetError;
use dns_lookup::lookup_host;
use std::net::IpAddr;

// Documentation imports
#[cfg(doc)]
use super::{IcmpTarget, TcpTarget};

/// A ResolvePolicy allows control over IP address resolution of network targets
/// like [IcmpTarget] and [TcpTarget].
#[derive(PartialEq, Debug)]
pub enum ResolvePolicy {
    /// Resolve use all IP address versions
    Agnostic,
    /// Resolve to IPv4 addresses only
    ResolveToIPv4,
    /// Resolve to IPv6 addresses only
    ResolveToIPv6,
}

impl ResolvePolicy {
    /// Resolve given "fully qualified domain name" (fancy name for a hostname or ip address)
    /// to a series of ip addresses associated with given fqhn.
    ///
    /// # Arguments
    /// * fqhn: string containing "fully qualified domain name" e.g. "::1", "localhost".
    ///
    /// # Returns
    /// * On success, vector containing all ip addresses the fqhn resolved to.
    /// * On failure, a [ResolveTargetError]. Either failed the name resolution itself or all addresses were filtered
    ///   out according to [ResolvePolicy].
    ///
    /// # Example
    /// ```
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # use mempool_space::ResolvePolicy;
    ///
    /// // FQHN was resolved
    /// assert_eq!(
    ///     ResolvePolicy::Agnostic.resolve("127.0.0.1").unwrap(),
    ///     vec![IpAddr::V4(Ipv4Addr::LOCALHOST)]
    /// );
    ///
    /// // FQHN was resolved, but all Addresses were filtered
    /// assert_eq!(ResolvePolicy::ResolveToIPv6.resolve("127.0.0.1").is_err(), true);
    /// ```
    pub fn resolve(&self, fqhn: &str) -> Result<Vec<IpAddr>, ResolveTargetError> {
        let mut addrs = lookup_host(fqhn)?;

        addrs = match &self {
            ResolvePolicy::Agnostic => addrs,
            ResolvePolicy::ResolveToIPv4 => addrs.into_iter().filter(|ip| ip.is_ipv4()).collect(),
            ResolvePolicy::ResolveToIPv6 => addrs.into_iter().filter(|ip| ip.is_ipv6()).collect(),
        };

        if addrs.is_empty() {
            Err(ResolveTargetError::from("Given Policy filtered all resolved addresses"))
        } else {
            Ok(addrs)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::*;

    #[test]
    fn resolver_policy_agnostic() {
        // Expectency: If ResolvePolicy is agnostic, resolve can return
        // IPv4 and IPv6 addresses if the resolution was successfull
        let policy = ResolvePolicy::Agnostic;

        // Test if ipv4 localhost is resolvable
        let ipv4_localhost = String::from("127.0.0.1");
        let res = policy.resolve(&ipv4_localhost).unwrap();
        assert_eq!(res, vec![IpAddr::V4(Ipv4Addr::LOCALHOST)]);

        // Test if ipv6 localhost is resolvable
        let ipv6_localhost = String::from("::1");
        let res = policy.resolve(&ipv6_localhost).unwrap();
        assert_eq!(res, vec![IpAddr::V6(Ipv6Addr::LOCALHOST)]);
    }

    #[test]
    fn resolver_policy_ipv4() {
        // Expectency: If ResolvePolicy is set to IPv4, resolve returns
        // only IPv4 addresses if the resolution was successfull
        let policy = ResolvePolicy::ResolveToIPv4;

        // Test if ipv4 localhost is resolvable
        let ipv4_localhost = String::from("127.0.0.1");
        let res = policy.resolve(&ipv4_localhost).unwrap();
        assert_eq!(res, vec![IpAddr::V4(Ipv4Addr::LOCALHOST)]);

        // Test if ipv6 localhost is resolvable. IPv6 addresses must be filtered out
        let ipv6_localhost = String::from("::1");
        assert_eq!(
            format!("{}", policy.resolve(&ipv6_localhost).unwrap_err()),
            "Given Policy filtered all resolved addresses"
        );
    }

    #[test]
    fn resolver_policy_ipv6() {
        // Expectency: If ResolvePolicy is set to IPv6, resolve can return
        // only IPv6 addresses if the resolution was successfull
        let policy = ResolvePolicy::ResolveToIPv6;

        // Test if ipv6 localhost is resolvable
        let ipv6_localhost = String::from("::1");
        let res = policy.resolve(&ipv6_localhost).unwrap();
        assert_eq!(res, vec![IpAddr::V6(Ipv6Addr::LOCALHOST)]);

        // Test if ipv4 localhost is resolvable. IPv4 addresses must be filtered out
        let ipv4_localhost = String::from("127.0.0.1");
        assert_eq!(
            format!("{}", policy.resolve(&ipv4_localhost).unwrap_err()),
            "Given Policy filtered all resolved addresses"
        );
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn resolver_policy_fail_to_resolve() {
        // Expectency: If ResolvePolicy must return an io::Error if the given hostname
        // can't be resolved.
        let policy = ResolvePolicy::Agnostic;

        let invalid_host = String::from("askjdakdsjhaksd.com");

        assert_eq!(
            format!("{}", policy.resolve(&invalid_host).unwrap_err()),
            "IoError caused by: failed to lookup address information: Name or service not known"
        );
    }
}
