//! UDP abstractions used by the tunneling implementation.

pub mod udp_server;
pub mod udp_tunnel;

use byte_pool::Block;
use std::net::SocketAddr;
use tokio::sync::mpsc::{Receiver, Sender};

/// Message types used by the UDP server/tunnel tasks.
pub enum UdpMessage {
    /// A datagram with metadata about source/destination.
    Packet(UdpPacket),
    /// Request the receiver to shut down.
    Quit,
}

/// Sender half of the UDP message channel.
pub type UdpSender = Sender<UdpMessage>;
/// Receiver half of the UDP message channel.
pub type UdpReceiver = Receiver<UdpMessage>;

/// UDP datagram payload and addressing info.
pub struct UdpPacket {
    /// Backed by a shared byte pool to reduce allocations.
    pub payload: Block<'static, Vec<u8>>,
    /// Local socket address the packet arrived on or will be sent to.
    pub local_addr: SocketAddr,
    /// Optional peer address (None when not applicable).
    pub peer_addr: Option<SocketAddr>,
}
