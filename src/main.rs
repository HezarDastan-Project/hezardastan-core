//! Main entry point for the HezarDastan Core server.
//! This module handles listening for incoming connections and dispatching them
//! to the appropriate obfuscated protocols (OTLS/WS, AOQUIC).

use tokio::net::{TcpListener, UdpSocket};
use std::io;
use tracing::{info, error, debug};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

// Import the ObfuscatedProtocol trait and specific protocol modules
use crate::protocols::{otls_ws, aoquic, ObfuscatedProtocol};

#[tokio::main]
async fn main() -> io::Result<()> {
    // --- Setup Tracing (Logging) ---
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("HezarDastan Core is starting up...");

    // --- Configuration (will be loaded from config file later) ---
    let tcp_listen_addr = "0.0.0.0:8443"; // Default port for OTLS/WS (mimics HTTPS)
    let udp_listen_addr = "0.0.0.0:8444"; // Default port for AOQUIC

    // --- Initialize Protocols ---
    // Create instances of our obfuscated protocols.
    // In a real scenario, these might be configured with specific keys/settings.
    let otls_ws_protocol = otls_ws::OtlsWsProtocol::new();
    let aoquic_protocol = aoquic::AoQuicProtocol::new();

    // --- Start TCP Listener for OTLS/WS ---
    let tcp_listener = TcpListener::bind(tcp_listen_addr).await
        .map_err(|e| {
            error!("Failed to bind TCP listener on {}: {}", tcp_listen_addr, e);
            e
        })?;
    info!("Listening for OTLS/WS connections on {}", tcp_listen_addr);

    // Spawn a task to handle incoming TCP connections
    tokio::spawn(async move {
        loop {
            match tcp_listener.accept().await {
                Ok((socket, peer_addr)) => {
                    info!("OTLS/WS: New TCP connection from {}", peer_addr);
                    // Clone the protocol instance to move into the spawned task
                    // (Alternatively, use Arc<Self> if protocol has shared state)
                    let protocol_instance = otls_ws_protocol.clone(); // Assuming .clone() is implemented for the protocol struct
                    tokio::spawn(async move {
                        if let Err(e) = protocol_instance.handle_tcp_stream(socket).await {
                            error!("OTLS/WS: Error handling TCP stream from {}: {}", peer_addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("OTLS/WS: TCP accept error: {}", e);
                }
            }
        }
    });

    // --- Start UDP Listener for AOQUIC ---
    let udp_socket = UdpSocket::bind(udp_listen_addr).await
        .map_err(|e| {
            error!("Failed to bind UDP socket on {}: {}", udp_listen_addr, e);
            e
        })?;
    info!("Listening for AOQUIC connections on {}", udp_listen_addr);

    // Spawn a task to handle incoming UDP packets
    // Note: For UDP, the socket itself needs to be shared or cloned carefully
    // For simplicity, we'll pass a reference to the socket for now,
    // but real QUIC implementations manage their own socket state.
    let aoquic_socket = udp_socket.into_std().expect("Failed to convert to std socket");
    aoquic_socket.set_nonblocking(true).expect("Failed to set non-blocking");
    let aoquic_socket = UdpSocket::from_std(aoquic_socket).expect("Failed to convert back to tokio socket");

    tokio::spawn(async move {
        let mut buf = vec![0u8; 65536]; // Max UDP packet size
        loop {
            match aoquic_socket.recv_from(&mut buf).await {
                Ok((len, peer_addr)) => {
                    debug!("AOQUIC: New UDP packet from {} ({} bytes)", peer_addr, len);
                    // Clone the protocol instance
                    let protocol_instance = aoquic_protocol.clone(); // Assuming .clone() is implemented
                    let packet_data = buf[..len].to_vec(); // Copy packet data for the spawned task
                    tokio::spawn(async move {
                        if let Err(e) = protocol_instance.handle_udp_packet(&aoquic_socket, &packet_data, peer_addr).await {
                            error!("AOQUIC: Error handling UDP packet from {}: {}", peer_addr, e);
                        }
                    });
                }
                Err(e) => {
                    // Handle WouldBlock error specifically for non-blocking UDP socket
                    if e.kind() != io::ErrorKind::WouldBlock {
                        error!("AOQUIC: UDP recv_from error: {}", e);
                    }
                }
            }
        }
    });

    info!("HezarDastan Core is running. Press Ctrl+C to stop.");
    std::future::pending::<()>().await;

    Ok(())
}
