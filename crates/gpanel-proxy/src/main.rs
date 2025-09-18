use clap::Parser;
use gpanel_core::{GhostPanelConfig, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};

mod proxy;
mod quic_server;
mod http_fallback;

use proxy::GhostProxy;

#[derive(Parser)]
#[command(name = "gpanel-proxy")]
#[command(about = "QUIC-based socket proxy for GhostPanel edge networking")]
struct Args {
    /// QUIC server bind address
    #[arg(long, default_value = "0.0.0.0:9443")]
    quic_addr: SocketAddr,

    /// HTTP/1.1 fallback server bind address
    #[arg(long, default_value = "0.0.0.0:9080")]
    http_addr: SocketAddr,

    /// Target Bolt API endpoint
    #[arg(long, default_value = "bolt://localhost:8080")]
    bolt_api: String,

    /// TLS certificate path (optional, generates self-signed in dev)
    #[arg(long)]
    cert_path: Option<String>,

    /// TLS private key path (optional)
    #[arg(long)]
    key_path: Option<String>,

    /// Enable development mode (allows insecure connections)
    #[arg(long)]
    dev_mode: bool,

    /// Maximum concurrent connections
    #[arg(long, default_value = "1000")]
    max_connections: usize,

    /// Connection idle timeout in seconds
    #[arg(long, default_value = "300")]
    idle_timeout: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .init();

    let args = Args::parse();

    info!("🚀 Starting GhostPanel QUIC Proxy");
    info!("   QUIC/HTTP3 server: {}", args.quic_addr);
    info!("   HTTP/1.1 fallback: {}", args.http_addr);
    info!("   Target Bolt API: {}", args.bolt_api);
    info!("   Development mode: {}", args.dev_mode);

    let config = GhostPanelConfig {
        web_port: args.quic_addr.port(),
        agent_port: 8000, // Fixed for now
        cli_port: 9000,   // Fixed for now
        bolt_api_url: args.bolt_api.clone(),
        enable_quic: true,
        enable_http3: true,
        tls_cert_path: args.cert_path.clone(),
        tls_key_path: args.key_path.clone(),
        registries: Vec::new(), // No registries needed for proxy
    };

    // Create the proxy instance
    let proxy = Arc::new(GhostProxy::new(config, args.dev_mode, args.max_connections, args.idle_timeout).await?);

    // Start QUIC/HTTP3 server
    let quic_proxy = proxy.clone();
    let quic_task = tokio::spawn(async move {
        if let Err(e) = quic_proxy.serve_quic(args.quic_addr).await {
            error!("QUIC server error: {}", e);
        }
    });

    // Start HTTP/1.1 fallback server
    let http_proxy = proxy.clone();
    let http_task = tokio::spawn(async move {
        if let Err(e) = http_proxy.serve_http(args.http_addr).await {
            error!("HTTP fallback server error: {}", e);
        }
    });

    info!("✅ GhostPanel QUIC Proxy started successfully");
    info!("🌐 Access via QUIC/HTTP3: https://{}", args.quic_addr);
    info!("🔄 HTTP/1.1 fallback: http://{}", args.http_addr);

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(_) => {
            info!("🛑 Shutdown signal received, stopping proxy...");
        }
        Err(e) => {
            error!("Failed to listen for shutdown signal: {}", e);
        }
    }

    // Graceful shutdown
    quic_task.abort();
    http_task.abort();

    info!("👋 GhostPanel QUIC Proxy stopped");
    Ok(())
}