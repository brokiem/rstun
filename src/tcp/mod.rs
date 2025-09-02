//! TCP abstractions used by the tunneling implementation.
//!
//! This module provides traits and types for handling TCP streams in a
//! generic way, allowing for different stream types to be used interchangeably.
//! It is used by the tunneling implementation to manage incoming and outgoing
//! TCP connections.

use std::net::SocketAddr;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};

pub mod tcp_server;
pub mod tcp_tunnel;

/// Async stream abstraction over tokio's TcpStream, used to generalize over
/// different stream types in tunnels.
pub trait AsyncStream: AsyncRead + AsyncWrite + Unpin + Send + 'static {
    fn peer_addr(&self) -> std::io::Result<SocketAddr>;
}

impl AsyncStream for TcpStream {
    fn peer_addr(&self) -> std::io::Result<SocketAddr> {
        TcpStream::peer_addr(self)
    }
}

/// Request to process an inbound stream and optionally its intended destination.
pub struct StreamRequest<S: AsyncStream> {
    pub stream: S,
    pub dst_addr: Option<SocketAddr>,
}

/// Messages passed between TCP server and tunnel task.
pub enum StreamMessage<S: AsyncStream> {
    Request(StreamRequest<S>),
    Quit,
}

/// Sender half of the TCP request channel.
pub type StreamSender<S> = Sender<StreamMessage<S>>;
/// Receiver half of the TCP request channel.
pub type StreamReceiver<S> = Receiver<StreamMessage<S>>;
