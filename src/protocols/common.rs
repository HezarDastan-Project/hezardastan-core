//! This module defines common structures and utilities shared across various
//! protocols and modules within the HezarDastan Core.

use std::fmt;

/// Represents a generic error that can occur within the HezarDastan core protocols.
#[derive(Debug)]
pub enum ProtocolError {
    Io(std::io::Error),
    /// An error occurred during protocol handshake or negotiation.
    HandshakeError(String),
    /// Data obfuscation/deobfuscation failed.
    ObfuscationError(String),
    /// An unexpected protocol state or data format was encountered.
    ProtocolViolation(String),
    /// General error with a descriptive message.
    Other(String),
}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> Self {
        ProtocolError::Io(err)
    }
}

// Allow ProtocolError to be printed easily
impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolError::Io(err) => write!(f, "IO Error: {}", err),
            ProtocolError::HandshakeError(msg) => write!(f, "Handshake Error: {}", msg),
            ProtocolError::ObfuscationError(msg) => write!(f, "Obfuscation Error: {}", msg),
            ProtocolError::ProtocolViolation(msg) => write!(f, "Protocol Violation: {}", msg),
            ProtocolError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

/// `TunnelConfig` defines configuration parameters for a specific tunnel connection.
/// This will be passed from the panel/client to the core.
#[derive(Debug, Clone)]
pub struct TunnelConfig {
    pub server_address: String,
    pub server_port: u16,
    /// The UUID or unique identifier for the user/connection.
    pub user_id: String, 
    /// Specifies the protocol to use (e.g., "otls-ws", "aoquic").
    pub protocol_type: ProtocolType,
    /// Optional parameters specific to the chosen protocol.
    pub protocol_params: std::collections::HashMap<String, String>,
    /// Indicates if Kill Switch should be active for this tunnel.
    pub enable_kill_switch: bool,
    /// Domain to mimic for obfuscation (e.g., "www.google.com").
    pub mimic_domain: String,
}

/// `ProtocolType` enumerates the different obfuscated protocols supported by HezarDastan.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProtocolType {
    OtlsWs,
    AoQuic,
    // Add more protocols here as we develop them
}

impl ProtocolType {
    /// Converts a string into a ProtocolType enum.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "otls-ws" => Some(ProtocolType::OtlsWs),
            "aoquic" => Some(ProtocolType::AoQuic),
            _ => None,
        }
    }

    /// Converts ProtocolType enum into a string.
    pub fn to_string_repr(&self) -> &'static str {
        match self {
            ProtocolType::OtlsWs => "otls-ws",
            ProtocolType::AoQuic => "aoquic",
        }
    }
}

/// Represents a data packet with optional metadata, used for internal communication.
#[derive(Debug, Clone)]
pub struct Packet {
    pub data: Vec<u8>,
    /// Optional metadata, e.g., for routing or session management.
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

// Helper function to create a basic Packet
impl Packet {
    pub fn new(data: Vec<u8>) -> Self {
        Packet {
            data,
            metadata: None,
        }
    }
}
