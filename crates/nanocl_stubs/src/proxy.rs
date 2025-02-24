#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Proxy rules modes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum ProxyRule {
  /// Redirect http trafic
  Http(ProxyRuleHttp),
  /// Redirect tcp and udp trafic
  Stream(Vec<ProxyRuleStream>),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ProxySslConfig {
  /// Path to the certificate
  pub certificate: String,
  /// Path to the certificate key
  pub certificate_key: String,
  /// Path to the dhparam file
  pub dh_param: Option<String>,
}

/// Defines a proxy rule target
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct CargoTarget {
  /// The cargo key
  pub key: String,
  /// The cargo port
  pub port: u16,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum UrlRedirect {
  MovedPermanently,
  PermanentRedirect,
  TemporaryRedirect,
  // TODO ?
  // Found,
  // SeeOther,
}

impl std::fmt::Display for UrlRedirect {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UrlRedirect::MovedPermanently => write!(f, "301"),
      UrlRedirect::PermanentRedirect => write!(f, "308"),
      UrlRedirect::TemporaryRedirect => write!(f, "307"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct HttpTarget {
  /// Url to target
  pub url: String,
  /// Redirect type if it's a redirect
  pub redirect: Option<UrlRedirect>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum LocationTarget {
  /// Target an existing cargo
  Cargo(CargoTarget),
  /// Target a specific http url
  Http(HttpTarget),
  /// Target a specific unix socket
  Unix(String),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct UriTarget {
  /// Uri to target
  pub uri: String,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum StreamTarget {
  /// Target an existing cargo
  Cargo(CargoTarget),
  /// Target a specific uri
  Uri(UriTarget),
  /// Target a specific unix socket
  Unix(String),
}

/// Proxy rules modes
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub enum ProxyStreamProtocol {
  Tcp,
  Udp,
}

/// Proxy rules modes
#[derive(Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ProxyRuleStream {
  /// Type of the network binding private | public | internal | namespace:$namespace_name
  pub network: String,
  /// Protocol to use Tcp | Udp
  pub protocol: ProxyStreamProtocol,
  /// The port to open on nodes
  pub port: u16,
  /// The ssl configuration
  pub ssl: Option<ProxySslConfig>,
  /// The target
  pub target: StreamTarget,
}

/// Defines a proxy rule location
#[derive(Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ProxyHttpLocation {
  /// The path
  pub path: String,
  /// The target cargo
  pub target: LocationTarget,
}

/// Defines a proxy rule http config
#[derive(Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ProxyRuleHttp {
  /// The domain
  pub domain: Option<String>,
  /// Type of private | public | internal | namespace:$namespace_name
  pub network: String,
  /// The locations to handle multiple paths
  pub locations: Vec<ProxyHttpLocation>,
  /// The ssl configuration
  pub ssl: Option<ProxySslConfig>,
  /// Path to extra config file to include
  pub includes: Option<Vec<String>>,
}

/// Define proxy rules to apply
#[derive(Debug, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
pub struct ResourceProxyRule {
  /// Cargo to watch for changes
  pub watch: Vec<String>,
  /// The rule
  pub rule: ProxyRule,
}
