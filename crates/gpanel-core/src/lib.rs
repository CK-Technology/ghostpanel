pub mod api;
pub mod bolt;
pub mod config;
pub mod container;
pub mod error;
pub mod quic;
pub mod registry;

pub use error::{Error, Result};
pub use container::*;
pub use registry::*;
pub use bolt::*;

/// Core types and utilities shared across GhostPanel components
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GhostPanelConfig {
    pub web_port: u16,
    pub agent_port: u16,
    pub cli_port: u16,
    pub bolt_api_url: String,
    pub enable_quic: bool,
    pub enable_http3: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    pub registries: Vec<RegistryConfig>,
}

impl Default for GhostPanelConfig {
    fn default() -> Self {
        Self {
            web_port: 9443,
            agent_port: 8000,
            cli_port: 9000,
            bolt_api_url: "bolt://localhost:8080".to_string(),
            enable_quic: true,
            enable_http3: true,
            tls_cert_path: None,
            tls_key_path: None,
            registries: vec![
                // Default local Drift registry
                RegistryConfig {
                    name: "local-drift".to_string(),
                    url: "http://localhost:5000".to_string(),
                    username: None,
                    password: None,
                    insecure: true,
                },
                // Docker Hub for public images
                RegistryConfig {
                    name: "docker-hub".to_string(),
                    url: "https://registry-1.docker.io".to_string(),
                    username: None,
                    password: None,
                    insecure: false,
                },
            ],
        }
    }
}