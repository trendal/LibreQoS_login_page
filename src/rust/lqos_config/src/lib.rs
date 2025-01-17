//! The `lqos_config` crate stores and handles LibreQoS configuration.
//! Configuration is drawn from:
//! * The `ispConfig.py` file.
//! * The `/etc/lqos.conf` file.
//! * `ShapedDevices.csv` files.
//! * `network.json` files.

#![warn(missing_docs)]
mod authentication;
mod etc;
mod libre_qos_config;
mod program_control;
mod shaped_devices;

pub use authentication::{UserRole, WebUsers};
pub use etc::{BridgeConfig, BridgeInterface, BridgeVlan, EtcLqos, Tunables};
pub use libre_qos_config::LibreQoSConfig;
pub use program_control::load_libreqos;
pub use shaped_devices::{ConfigShapedDevices, ShapedDevice};

/// Used as a constant in determining buffer preallocation
pub const SUPPORTED_CUSTOMERS: usize = 16_000_000;
