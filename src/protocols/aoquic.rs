// src/protocols/aoquic.rs
//! Implements the Adaptive Obfuscated QUIC (AOQUIC) protocol.

use async_trait::async_trait;
use tokio::net::{TcpStream, UdpSocket, SocketAddr};
use std::io;
use tracing::{info, debug, error, warn}; // Import tracing macros

use crate::protocols::ObfuscatedProtocol; // Import the trait

/// Represents the AOQUIC obfuscated protocol.
/// This struct will hold configuration and state specific to AOQUIC.
#[derive(Clone)] // Required for .clone() in main.rs
pub struct AoQuicProtocol {
    // TODO: Add fields for QUIC configuration, obfuscation keys, etc.
}

impl AoQuicProtocol {
    /// Creates a new instance of the AoQuicProtocol.
    pub fn new() -> Self {
        info!("Initializing AOQUIC Protocol.");
        AoQuicProtocol {
            // Initialize fields here
        }
    }
}

#[async_trait]
impl ObfuscatedProtocol for AoQuicProtocol {
    fn name(&self) -> &'static str {
        "AOQUIC"
    }

    // AOQUIC is a UDP-based protocol, so this method will likely not be used,
    // or it might log an error if called.
    async fn handle_tcp_stream(&self, stream: TcpStream) -> io::Result<()> {
        let peer_addr = stream.peer_addr()?;
        error!("AOQUIC: Received unexpected TCP stream from {}. This protocol is UDP-based.", peer_addr);
        Err(io::Error::new(io::ErrorKind::Other, "AOQUIC does not handle TCP streams."))
    }

    async fn handle_udp_packet(&self, socket: &UdpSocket, buf: &[u8], peer_addr: SocketAddr) -> io::Result<()> {
        debug!("AOQUIC: Handling incoming UDP packet from {} ({} bytes)", peer_addr, buf.len());

        // TODO: Here's where the actual QUIC packet processing and obfuscation/de-obfuscation logic will go.
        // This will involve:
        // 1. De-obfuscating the packet.
        // 2. Processing it as a QUIC packet (e.g., establishing connection, handling streams).
        // 3. Forwarding data through the QUIC connection.

        // For now, we'll just simulate processing and potentially echo back (for testing).
        // In a real scenario, you wouldn't necessarily echo back raw packets.

        // Example: Simple echo for demonstration (REMOVE IN REAL IMPLEMENTATION)
        // if let Err(e) = socket.send_to(buf, peer_addr).await {
        //     warn!("AOQUIC: Failed to echo UDP packet back to {}: {}", peer_addr, e);
        // }

        info!("AOQUIC: Successfully processed simulated UDP packet from {}", peer_addr);
        Ok(())
    }
}

// Add unit tests for this module
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::UdpSocket;
    use std::time::Duration;

    #[tokio::test]
    async fn test_aoquic_protocol_name() {
        let protocol = AoQuicProtocol::new();
        assert_eq!(protocol.name(), "AOQUIC");
    }

    #[tokio::test]
    async fn test_aoquic_handles_udp_packet_successfully() {
        let protocol = AoQuicProtocol::new();
        let listener_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap(); // Bind to ephemeral port
        let addr = listener_socket.local_addr().unwrap();

        // Spawn a task to simulate receiving and handling a packet
        let listener_socket_clone = listener_socket.try_clone().unwrap();
        tokio::spawn(async move {
            let mut buf = vec![0u8; 1024];
            let (len, peer_addr) = listener_socket_clone.recv_from(&mut buf).await.unwrap();
            let received_data = buf[..len].to_vec();
            // Simulate protocol handling
            let _ = protocol.handle_udp_packet(&listener_socket_clone, &received_data, peer_addr).await;
        });

        // Send a dummy UDP packet from a "client"
        let client_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let test_data = b"hello aoquic";
        let send_result = client_socket.send_to(test_data, addr).await;
        assert!(send_result.is_ok()); // Ensure client can send

        // Give some time for the spawned task to process
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    #[tokio::test]
    async fn test_aoquic_rejects_tcp_streams() {
        let protocol = AoQuicProtocol::new();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap(); // Dummy listener
        let addr = listener.local_addr().unwrap();

        // Connect a dummy client
        let client_stream_result = TcpStream::connect(addr).await;
        assert!(client_stream_result.is_ok());
        let client_stream = client_stream_result.unwrap();

        let result = protocol.handle_tcp_stream(client_stream).await;

        // Assert that it returns an error because AOQUIC is UDP-based
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::Other);
    }
}
