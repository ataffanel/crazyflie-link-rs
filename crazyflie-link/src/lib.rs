#[macro_use]
extern crate bitflags;

mod connection;
mod context;
mod error;
mod packet;

#[cfg(all(feature = "native", feature = "webusb"))]
compile_error!("feature \"native\" and feature \"webusb\" cannot be enabled at the same time");

#[cfg(feature = "native")]
pub(crate) use crazyradio as crazyradio;
#[cfg(feature = "webusb")]
pub(crate) use crazyradio_webusb as crazyradio;

pub use connection::{Connection, ConnectionStatus};
pub use context::LinkContext;
pub use packet::Packet;
pub use error::Error;
