// src/protocols/mod.rs
//! Defines traits and modules for various obfuscated protocols used by HezarDastan Core.

use async_trait::async_trait; // For async traits
use tokio::net::{TcpStream, UdpSocket, SocketAddr};
use std::io;

// Re-export specific protocol modules
pub mod otls_ws;
pub mod aoquic;

/// A trait defining the common interface for all obfuscated protocols.
/// Each protocol implementation must adhere to this interface.
#[async_trait]
pub trait ObfuscatedProtocol {
    /// Returns the name of the protocol (e.g., "OTLS/WS", "AOQUIC").
    fn name(&self) -> &'static str;

    /// Handles an incoming TCP stream for connection-oriented protocols.
    /// This method should perform the obfuscation handshake and then tunnel the traffic.
    async fn handle_tcp_stream(&self, stream: TcpStream) -> io::Result<()>;

    /// Handles an incoming UDP packet for connectionless protocols.
    /// This method should de-obfuscate the packet and potentially forward it.
    async fn handle_udp_packet(&self, socket: &UdpSocket, buf: &[u8], peer_addr: SocketAddr) -> io::Result<()>;

    // TODO: Add methods for protocol-specific configuration, metrics, etc.
    // For example:
    // fn get_config(&self) -> &ProtocolConfig;
    // fn update_config(&mut self, new_config: ProtocolConfig);
}
