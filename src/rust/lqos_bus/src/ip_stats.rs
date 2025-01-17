use crate::TcHandle;
use serde::{Deserialize, Serialize};

/// Transmission representation of IP statistics associated
/// with a host.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IpStats {
  /// The host's IP address, as detected by the XDP program.
  pub ip_address: String,

  /// The current bits-per-second passing through this host. Tuple
  /// 0 is download, tuple 1 is upload.
  pub bits_per_second: (u64, u64),

  /// The current packets-per-second passing through this host. Tuple
  /// 0 is download, tuple 1 is upload.
  pub packets_per_second: (u64, u64),

  /// Median TCP round-trip-time for this host at the current time.
  pub median_tcp_rtt: f32,

  /// Associated TC traffic control handle.
  pub tc_handle: TcHandle,
}

/// Represents an IP Mapping in the XDP IP to TC/CPU mapping system.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct IpMapping {
  /// The mapped IP address. May be IPv4, or IPv6.
  pub ip_address: String,

  /// The CIDR prefix length of the host. Equivalent to the CIDR value
  /// after the /. e.g. `/24`.
  pub prefix_length: u32,

  /// The current TC traffic control handle.
  pub tc_handle: TcHandle,

  /// The CPU index associated with this IP mapping.
  pub cpu: u32,
}

/// Provided for backwards compatibility with `xdp_pping`, with the intent
/// to retire it eventually.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct XdpPpingResult {
  /// The TC handle in text format. e.g. "1:12"
  pub tc: String,

  /// The average (mean) RTT value for the current sample.
  pub avg: f32,

  /// The minimum RTT value for the current sample.
  pub min: f32,

  /// The maximum RTT value for the current sample.
  pub max: f32,

  /// The median RTT value for the current sample.
  pub median: f32,

  /// The number of samples from which these values were
  /// derived. If 0, the other values are invalid.
  pub samples: u32,
}
