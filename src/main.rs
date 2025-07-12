//! Main entry point for the HezarDastan Core server.
//! This module handles listening for incoming connections and dispatching them
//! to the appropriate obfuscated protocols (OTLS/WS, AOQUIC).

use tokio::net::{TcpListener, UdpSocket};
use std::io;

// Import our protocol modules (these will be used later)
// use hezardastan_core::protocols::{otls_ws, aoquic};
// use hezardastan_core::security::{kill_switch, traffic_obfuscation};
// use hezardastan_core::utils::{logging, config};


#[tokio::main]
async fn main() -> io::Result<()> {
    println!("HezarDastan Core is starting...");

    // --- Configuration (will be loaded from config file later) ---
    let tcp_listen_addr = "0.0.0.0:8443"; // Default port for OTLS/WS (mimics HTTPS)
    let udp_listen_addr = "0.0.0.0:8444"; // Default port for AOQUIC

    // --- Start TCP Listener for OTLS/WS ---
    let tcp_listener = TcpListener::bind(tcp_listen_addr).await?;
    println!("Listening for OTLS/WS connections on {}", tcp_listen_addr);

    tokio::spawn(async move {
        loop {
            match tcp_listener.accept().await {
                Ok((socket, peer_addr)) => {
                    println!("OTLS/WS: New TCP connection from {}", peer_addr);
                    // TODO: Handle the incoming TCP socket with OTLS/WS protocol logic
                    // For now, we just accept and close.
                    // otls_ws::handle_connection(socket).await; // This will be implemented later
                }
                Err(e) => eprintln!("OTLS/WS: TCP accept error: {}", e),
            }
        }
    });

    // --- Start UDP Listener for AOQUIC ---
    // UDP is connectionless, so we bind a single socket and listen for packets.
    let udp_socket = UdpSocket::bind(udp_listen_addr).await?;
    println!("Listening for AOQUIC connections on {}", udp_listen_addr);

    tokio::spawn(async move {
        let mut buf = vec![0u8; 65536]; // Max UDP packet size
        loop {
            match udp_socket.recv_from(&mut buf).await {
                Ok((len, peer_addr)) => {
                    println!("AOQUIC: New UDP packet from {} ({} bytes)", peer_addr, len);
                    // TODO: Handle the incoming UDP packet with AOQUIC protocol logic
                    // This will involve dispatching to the correct QUIC connection.
                    // aoquic::handle_packet(&udp_socket, &buf[..len], peer_addr).await; // This will be implemented later
                }
                Err(e) => eprintln!("AOQUIC: UDP recv_from error: {}", e),
            }
        }
    });

    // Keep the main thread alive indefinitely
    // In a real application, this might be a signal handler or a long-running task.
    // For now, we'll just print a message and wait.
    println!("HezarDastan Core is running. Press Ctrl+C to stop.");
    // A simple way to keep the main task alive without consuming CPU
    // In a real server, you might use a channel to wait for a shutdown signal.
    std::future::pending::<()>().await;

    Ok(())
}
