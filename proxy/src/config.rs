use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

use url::{Host, HostAndPort, Url};

use convert::TryFrom;

// TODO:
//
// * Make struct fields private.

/// Tracks all configuration settings for the process.
#[derive(Clone, Debug)]
pub struct Config {
    /// Where to listen for connections that are initiated on the host.
    pub private_listener: Listener,

    /// Where to listen for connections initiated by external sources.
    pub public_listener: Listener,

    /// Where to listen for connectoins initiated by the control planey.
    pub control_listener: Listener,

    /// Where to forward externally received connections.
    pub private_forward: Option<Addr>,

    /// The maximum amount of time to wait for a connection to the public peer.
    pub public_connect_timeout: Option<Duration>,

    /// The maximum amount of time to wait for a connection to the private peer.
    pub private_connect_timeout: Duration,

    /// The path to "/etc/resolv.conf"
    pub resolv_conf_path: PathBuf,

    /// Where to talk to the control plane.
    pub control_host_and_port: HostAndPort,

    /// Event queue capacity.
    pub event_buffer_capacity: usize,

    /// Interval after which to flush metrics.
    pub metrics_flush_interval: Duration,

    /// Timeout after which to cancel telemetry reports.
    pub report_timeout: Duration,

    pub pod_name: Option<String>,
    pub pod_namespace: Option<String>,
    pub node_name: Option<String>,
}

/// Configuration settings for binding a listener.
///
/// TODO: Rename this to be more inline with the actual types.
#[derive(Clone, Debug)]
pub struct Listener {
    /// The address to which the listener should bind.
    pub addr: Addr,
}

/// A logical address. This abstracts over the various strategies for cross
/// process communication.
#[derive(Clone, Copy, Debug)]
pub struct Addr(SocketAddr);

/// Errors produced when loading a `Config` struct.
#[derive(Clone, Debug)]
pub enum Error {
    InvalidEnvVar
}

#[derive(Clone, Debug)]
pub enum ParseError {
    NotANumber,
    InvalidUnit,
    HostIsNotAnIpAddress,
    NotUnicode,
    UrlError(UrlError),
}

#[derive(Clone, Copy, Debug)]
pub enum UrlError {
    /// The URl is syntactically invalid according to general URL parsing rules.
    SyntaxError,

    /// The URL has a scheme that isn't supported.
    UnsupportedScheme,

    /// The URL is missing the host part.
    MissingHost,

    /// The URL is missing the port and there is no default port.
    MissingPort,

    /// The URL contains a path component that isn't "/", which isn't allowed.
    PathNotAllowed,

    /// The URL contains a fragment, which isn't allowed.
    FragmentNotAllowed,
}

/// The strings used to build a configuration.
pub trait Strings {
    /// Retrieves the value for the key `key`.
    ///
    /// `key` must be one of the `ENV_` values below.
    fn get(&self, key: &str) -> Result<Option<String>, Error>;
}

/// An implementation of `Strings` that reads the values from environment variables.
pub struct Env;

pub struct TestEnv {
    values: HashMap<&'static str, String>
}

// Environment variables to look at when loading the configuration
const ENV_EVENT_BUFFER_CAPACITY: &str = "CONDUIT_PROXY_EVENT_BUFFER_CAPACITY";
pub const ENV_METRICS_FLUSH_INTERVAL_SECS: &str = "CONDUIT_PROXY_METRICS_FLUSH_INTERVAL_SECS";
const ENV_REPORT_TIMEOUT_SECS: &str = "CONDUIT_PROXY_REPORT_TIMEOUT_SECS";
pub const ENV_PRIVATE_LISTENER: &str = "CONDUIT_PROXY_PRIVATE_LISTENER";
pub const ENV_PRIVATE_FORWARD: &str = "CONDUIT_PROXY_PRIVATE_FORWARD";
pub const ENV_PUBLIC_LISTENER: &str = "CONDUIT_PROXY_PUBLIC_LISTENER";
pub const ENV_CONTROL_LISTENER: &str = "CONDUIT_PROXY_CONTROL_LISTENER";
const ENV_PRIVATE_CONNECT_TIMEOUT: &str = "CONDUIT_PROXY_PRIVATE_CONNECT_TIMEOUT";
const ENV_PUBLIC_CONNECT_TIMEOUT: &str = "CONDUIT_PROXY_PUBLIC_CONNECT_TIMEOUT";

const ENV_NODE_NAME: &str = "CONDUIT_PROXY_NODE_NAME";
const ENV_POD_NAME: &str = "CONDUIT_PROXY_POD_NAME";
const ENV_POD_NAMESPACE: &str = "CONDUIT_PROXY_POD_NAMESPACE";

pub const ENV_CONTROL_URL: &str = "CONDUIT_PROXY_CONTROL_URL";
const ENV_RESOLV_CONF: &str = "CONDUIT_RESOLV_CONF";

// Default values for various configuration fields
const DEFAULT_EVENT_BUFFER_CAPACITY: usize = 10_000; // FIXME
const DEFAULT_METRICS_FLUSH_INTERVAL_SECS: u64 = 10;
const DEFAULT_REPORT_TIMEOUT_SECS: u64 = 10; // TODO: is this a reasonable default?
const DEFAULT_PRIVATE_LISTENER: &str = "tcp://127.0.0.1:4140";
const DEFAULT_PUBLIC_LISTENER: &str = "tcp://0.0.0.0:4143";
const DEFAULT_CONTROL_LISTENER: &str = "tcp://0.0.0.0:4190";
const DEFAULT_PRIVATE_CONNECT_TIMEOUT_MS: u64 = 20;
const DEFAULT_CONTROL_URL: &str = "tcp://proxy-api.conduit.svc.cluster.local:8086";
const DEFAULT_RESOLV_CONF: &str = "/etc/resolv.conf";

// ===== impl Config =====

impl<'a> TryFrom<&'a Strings> for Config {
    type Err = Error;
    /// Load a `Config` by reading ENV variables.
    fn try_from(strings: &Strings) -> Result<Self, Self::Err> {
        // Parse all the environment variables. `env_var` and `env_var_parse`
        // will log any errors so defer returning any errors until all of them
        // have been parsed.
        let private_listener_addr = parse(strings, ENV_PRIVATE_LISTENER, str::parse);
        let public_listener_addr = parse(strings, ENV_PUBLIC_LISTENER, str::parse);
        let control_listener_addr = parse(strings, ENV_CONTROL_LISTENER, str::parse);
        let private_forward = parse(strings, ENV_PRIVATE_FORWARD, str::parse);
        let public_connect_timeout = parse(strings, ENV_PUBLIC_CONNECT_TIMEOUT, parse_number);
        let private_connect_timeout = parse(strings, ENV_PRIVATE_CONNECT_TIMEOUT, parse_number);
        let resolv_conf_path = strings.get(ENV_RESOLV_CONF);
        let control_host_and_port = parse(strings, ENV_CONTROL_URL, parse_url);
        let event_buffer_capacity = parse(strings, ENV_EVENT_BUFFER_CAPACITY, parse_number);
        let metrics_flush_interval_secs =
            parse(strings, ENV_METRICS_FLUSH_INTERVAL_SECS, parse_number);
        let report_timeout = parse(strings, ENV_REPORT_TIMEOUT_SECS, parse_number);
        let pod_name = strings.get(ENV_POD_NAME);
        let pod_namespace = strings.get(ENV_POD_NAMESPACE);
        let node_name = strings.get(ENV_NODE_NAME);

        Ok(Config {
            private_listener: Listener {
                addr: private_listener_addr?
                    .unwrap_or_else(|| Addr::from_str(DEFAULT_PRIVATE_LISTENER).unwrap()),
            },
            public_listener: Listener {
                addr: public_listener_addr?
                    .unwrap_or_else(|| Addr::from_str(DEFAULT_PUBLIC_LISTENER).unwrap()),
            },
            control_listener: Listener {
                addr: control_listener_addr?
                    .unwrap_or_else(|| Addr::from_str(DEFAULT_CONTROL_LISTENER).unwrap()),
            },
            private_forward: private_forward?,
            public_connect_timeout: public_connect_timeout?.map(Duration::from_millis),
            private_connect_timeout:
                Duration::from_millis(private_connect_timeout?
                                          .unwrap_or(DEFAULT_PRIVATE_CONNECT_TIMEOUT_MS)),
            resolv_conf_path: resolv_conf_path?
                .unwrap_or(DEFAULT_RESOLV_CONF.into())
                .into(),
            control_host_and_port: control_host_and_port?
                .unwrap_or_else(|| parse_url(DEFAULT_CONTROL_URL).unwrap()),

            event_buffer_capacity: event_buffer_capacity?.unwrap_or(DEFAULT_EVENT_BUFFER_CAPACITY),
            metrics_flush_interval:
                Duration::from_secs(metrics_flush_interval_secs?
                                        .unwrap_or(DEFAULT_METRICS_FLUSH_INTERVAL_SECS)),
            report_timeout:
                Duration::from_secs(report_timeout?.unwrap_or(DEFAULT_REPORT_TIMEOUT_SECS)),
            pod_name: pod_name?,
            pod_namespace: pod_namespace?,
            node_name: node_name?,
        })
    }
}

// ===== impl Addr =====

impl FromStr for Addr {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_url(s)? {
            HostAndPort {
                host: Host::Ipv4(ip),
                port,
            } => Ok(Addr(SocketAddr::new(ip.into(), port))),
            HostAndPort {
                host: Host::Ipv6(ip),
                port,
            } => Ok(Addr(SocketAddr::new(ip.into(), port))),
            HostAndPort {
                host: Host::Domain(_),
                ..
            } => Err(ParseError::HostIsNotAnIpAddress),
        }
    }
}

impl From<Addr> for SocketAddr {
    fn from(addr: Addr) -> SocketAddr {
        addr.0
    }
}

// ===== impl Env =====

impl Strings for Env {
    fn get(&self, key: &str) -> Result<Option<String>, Error> {
        match env::var(key) {
            Ok(value) => Ok(Some(value)),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(env::VarError::NotUnicode(_)) => {
                error!("{} is not encoded in Unicode", key);
                Err(Error::InvalidEnvVar)
            }
        }
    }
}

// ===== impl TestEnv =====

impl TestEnv {
    pub fn new() -> Self {
        Self {
            values: Default::default(),
        }
    }

    pub fn put(&mut self, key: &'static str, value: String) {
        self.values.insert(key, value);
    }
}

impl Strings for TestEnv {
    fn get(&self, key: &str) -> Result<Option<String>, Error> {
        Ok(self.values.get(key).cloned())
    }
}

// ===== Parsing =====

fn parse_number<T>(s: &str) -> Result<T, ParseError> where T: FromStr {
    s.parse().map_err(|_| ParseError::NotANumber)
}

fn parse_url(s: &str) -> Result<HostAndPort, ParseError> {
    let url = Url::parse(&s).map_err(|_| ParseError::UrlError(UrlError::SyntaxError))?;
    let host = url.host()
        .ok_or_else(|| ParseError::UrlError(UrlError::MissingHost))?
        .to_owned();
    if url.scheme() != "tcp" {
        return Err(ParseError::UrlError(UrlError::UnsupportedScheme));
    }
    let port = url.port().ok_or_else(|| ParseError::UrlError(UrlError::MissingPort))?;
    if url.path() != "/" {
        return Err(ParseError::UrlError(UrlError::PathNotAllowed));
    }
    if url.fragment().is_some() {
        return Err(ParseError::UrlError(UrlError::FragmentNotAllowed));
    }
    Ok(HostAndPort {
        host,
        port,
    })
}

fn parse<T, Parse>(strings: &Strings, name: &str, parse: Parse) -> Result<Option<T>, Error>
    where Parse: FnOnce(&str) -> Result<T, ParseError> {
    match strings.get(name)? {
        Some(ref s) => {
            let r = parse(s).map_err(|parse_error| {
                error!("{} is not valid: {:?}", name, parse_error);
                Error::InvalidEnvVar
            })?;
            Ok(Some(r))
        },
        None => Ok(None),
    }
}

mod storage {
    use std::cmp::Ordering;
    use std::fmt;
    use std::str::FromStr;
    use std::marker::PhantomData;
    use super::{parse_number, ParseError};

    #[derive(Copy, Clone, Debug, Eq, Ord)]
    pub struct Storage<U: StorageUnit> {
        bytes: usize,
        unit: PhantomData<U>,
    }
    pub trait StorageUnit {
        const NAME: &'static str;
        const SHORT_NAME: &'static str;
        const BYTES_PER_UNIT: usize;
    }

    // ===== impl Storage =====

    impl<Unit: StorageUnit> fmt::Display for Storage<Unit> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let float_value = 
                (self.bytes as f64) / (Unit::BYTES_PER_UNIT as f64);

            write!(f,
                "{number} {name}{plural}",
                number=float_value,
                name=Unit::NAME,
                plural=if float_value == 1f64 { "" } else { "s" }
            )
        }
    }

    impl<Unit: StorageUnit> From<usize> for Storage<Unit> {
        fn from(u: usize) -> Self {
            Self {
                bytes: u * Unit::BYTES_PER_UNIT,
                unit: PhantomData
            }
        }
    }

    impl<A: StorageUnit> Storage<A> {
        pub fn into<B: StorageUnit>(self) -> Storage<B> {
            Storage {
                bytes: self.bytes,
                unit: PhantomData,
            }
        }
    }

    impl<A, B> PartialEq<Storage<B>> for Storage<A>
    where   
        A: StorageUnit,
        B: StorageUnit,
    {
        fn eq(&self, rhs: &Storage<B>) -> bool{
            self.bytes == rhs.bytes
        }
    }

    impl<A, B> PartialOrd<Storage<B>> for Storage<A>
    where   
        A: StorageUnit,
        B: StorageUnit,
    {
        fn partial_cmp(&self, rhs: &Storage<B>) -> Option<Ordering>{
            self.bytes.partial_cmp(&rhs.bytes)
        }
    }

    macro_rules! impl_ops {
        ($($trait:ident, $fun:ident ),+) => {
            $(
                impl<A, B> ::std::ops::$trait<Storage<B>> for Storage<A>
                where   
                    A: StorageUnit,
                    B: StorageUnit,
                {
                    type Output = Storage<A>;
                    fn $fun(self, rhs: Storage<B>) -> Storage<A> {
                        Storage {
                            bytes: self.bytes.$fun(rhs.bytes),
                            unit: PhantomData
                        }
                    }
                }
            )+
        }
    }

    macro_rules! mk_units {
        ($($name:ident, $long_name:expr, $short_name:expr, $bytes:expr),+) => {
            $(
                #[derive(Copy, Clone, Debug, Eq, PartialEq)]
                pub struct $name;

                impl StorageUnit for $name {
                    const NAME: &'static str = $long_name;
                    const SHORT_NAME: &'static str = $short_name;
                    const BYTES_PER_UNIT: usize = $bytes;
                }
            )+
        }
    }

    impl_ops! {
        Add, add,
        Sub, sub,
        Div, div,
        Mul, mul
    }

    mk_units!{
        Bytes,     "bytes"    , "B"  , 1,
        Kilobytes, "kilobytes", "KB" , 1_000,
        Kibibytes, "kibibytes", "KiB", 1_024,
        Megabytes, "megabytes", "MB" , 1_000_000,
        Mebibytes, "mebibytes", "MiB", 1_048_576,
        Gigabytes, "gigabytes", "GB" , 1_000_000_000,
        Gibibytes, "gibibytes", "GiB", 1_073_741_824
    }

    impl<U: StorageUnit> FromStr for Storage<U> {
        type Err = ParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let num_part = s.trim_matches(|c: char| !c.is_numeric()).trim();
            let unit_part = 
                s.trim_matches(char::is_numeric).trim()
                // NOTE: could save a string allocation by matching patterns 
                //       like `"B" | "b"`, but that ends up looking much uglier
                //       and this shouldn't be in the hot path...
                 .to_lowercase(); 
            let num: usize = parse_number(num_part)?;
            match unit_part.as_ref() {
                "b"   => Ok(Storage::<Bytes>::from(num).into::<U>()),
                "kb"  => Ok(Storage::<Kilobytes>::from(num).into::<U>()),
                "kib" => Ok(Storage::<Kibibytes>::from(num).into::<U>()),
                "mb"  => Ok(Storage::<Megabytes>::from(num).into::<U>()),
                "mib" => Ok(Storage::<Mebibytes>::from(num).into::<U>()),
                "gb"  => Ok(Storage::<Gigabytes>::from(num).into::<U>()),
                "gib" => Ok(Storage::<Gibibytes>::from(num).into::<U>()),
                unit => {
                    error!("invalid storage unit '{}'", unit);
                    Err(ParseError::InvalidUnit)
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn is_zero_cost() {
            use std::mem::size_of;
            assert_eq!(
                size_of::<Storage<Bytes>>(), size_of::<usize>()
            );
            assert_eq!(
                size_of::<Storage<Kilobytes>>(), size_of::<usize>()
            );
            assert_eq!(
                size_of::<Storage<Megabytes>>(), size_of::<usize>()
            );
            assert_eq!(
                size_of::<Storage<Gigabytes>>(), size_of::<usize>()
            );
        }

        #[test]
        fn parsing_simple() {
            assert_eq!(
                "15 GB".parse::<Storage<Gigabytes>>()
                       .expect("parse"),
                Storage::<Gigabytes>::from(15)
            );
            assert_eq!(
                "15 GiB".parse::<Storage<Gibibytes>>()
                       .expect("parse"),
                Storage::<Gibibytes>::from(15)
            );
        }


        #[test]
        fn parsing_does_unit_conversions() {
            assert_eq!(
                "1024 B".parse::<Storage<Kibibytes>>()
                       .expect("parse"),
                Storage::<Kibibytes>::from(1)
            );

            assert_eq!(
                "4096 KiB".parse::<Storage<Kibibytes>>()
                       .expect("parse"),
                Storage::<Mebibytes>::from(4)
            );
        }
    }


}