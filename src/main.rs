//! Main entry point for the HezarDastan Core server.
//! This module handles listening for incoming connections and dispatching them
//! to the appropriate obfuscated protocols (OTLS/WS, AOQUIC).

use tokio::net::{TcpListener, UdpSocket};
use std::io;
use tracing::{info, error, debug}; // Import logging macros
use tracing_subscriber::{EnvFilter, FmtSubscriber}; // For logging setup

// Import our protocol modules (these will be used later)
// use crate::protocols::{otls_ws, aoquic}; // Use `crate::` for internal modules
// use crate::security::{kill_switch, traffic_obfuscation};
// use crate::utils::{logging, config}; // If `logging` was a separate module, we'd use it here

#[tokio::main]
async fn main() -> io::Result<()> {
    // --- Setup Tracing (Logging) ---
    // Initialize the tracing subscriber to log events to stdout.
    // It reads the RUST_LOG environment variable for filtering (e.g., RUST_LOG=info)
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env()) // Read log level from RUST_LOG env var
        .init();

    info!("HezarDastan Core is starting up...");

    // --- Configuration (will be loaded from config file later) ---
    let tcp_listen_addr = "0.0.0.0:8443"; // Default port for OTLS/WS (mimics HTTPS)
    let udp_listen_addr = "0.0.0.0:8444"; // Default port for AOQUIC

    // --- Start TCP Listener for OTLS/WS ---
    let tcp_listener = TcpListener::bind(tcp_listen_addr).await
        .map_err(|e| {
            error!("Failed to bind TCP listener on {}: {}", tcp_listen_addr, e);
            e
        })?;
    info!("Listening for OTLS/WS connections on {}", tcp_listen_addr);

    tokio::spawn(async move {
        loop {
            match tcp_listener.accept().await {
                Ok((socket, peer_addr)) => {
                    info!("OTLS/WS: New TCP connection from {}", peer_addr);
                    // TODO: Handle the incoming TCP socket with OTLS/WS protocol logic
                    // For now, we just accept and close.
                    // otls_ws::handle_connection(socket).await; // This will be implemented later
                }
                Err(e) => {
                    error!("OTLS/WS: TCP accept error: {}", e);
                }
            }
        }
    });

    // --- Start UDP Listener for AOQUIC ---
    // UDP is connectionless, so we bind a single socket and listen for packets.
    let udp_socket = UdpSocket::bind(udp_listen_addr).await
        .map_err(|e| {
            error!("Failed to bind UDP socket on {}: {}", udp_listen_addr, e);
            e
        })?;
    info!("Listening for AOQUIC connections on {}", udp_listen_addr);

    tokio::spawn(async move {
        let mut buf = vec![0u8; 65536]; // Max UDP packet size
        loop {
            match udp_socket.recv_from(&mut buf).await {
                Ok((len, peer_addr)) => {
                    debug!("AOQUIC: New UDP packet from {} ({} bytes)", peer_addr, len); // Use debug for packet level logs
                    // TODO: Handle the incoming UDP packet with AOQUIC protocol logic
                    // This will involve dispatching to the correct QUIC connection.
                    // aoquic::handle_packet(&udp_socket, &buf[..len], peer_addr).await; // This will be implemented later
                }
                Err(e) => {
                    error!("AOQUIC: UDP recv_from error: {}", e);
                }
            }
        }
    });

    info!("HezarDastan Core is running. Press Ctrl+C to stop.");
    // Keep the main thread alive indefinitely to allow background tasks to run.
    // In a real application, this might be a signal handler or a long-running task.
    std::future::pending::<()>().await;

    Ok(())
}
