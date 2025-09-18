use gpanel_core::{GhostPanelConfig, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

use crate::quic_server::QuicProxyServer;
use crate::http_fallback::HttpFallbackServer;

/// Main GhostProxy instance that coordinates QUIC and HTTP services
pub struct GhostProxy {
    config: GhostPanelConfig,
    quic_server: QuicProxyServer,
    http_server: HttpFallbackServer,
    stats: Arc<RwLock<ProxyStats>>,
}

#[derive(Default, Debug, serde::Serialize)]
pub struct ProxyStats {
    pub active_connections: u64,
    pub total_requests: u64,
    pub quic_requests: u64,
    pub http_requests: u64,
    pub bytes_transferred: u64,
    pub uptime_seconds: u64,
}

impl GhostProxy {
    pub async fn new(
        config: GhostPanelConfig,
        dev_mode: bool,
        max_connections: usize,
        idle_timeout: u64,
    ) -> Result<Self> {
        info!("🔧 Initializing GhostPanel QUIC Proxy");

        let stats = Arc::new(RwLock::new(ProxyStats::default()));

        // Initialize QUIC server
        let quic_server = QuicProxyServer::new(
            config.clone(),
            dev_mode,
            max_connections,
            idle_timeout,
            stats.clone(),
        ).await?;

        // Initialize HTTP fallback server
        let http_server = HttpFallbackServer::new(
            config.clone(),
            stats.clone(),
        )?;

        Ok(Self {
            config,
            quic_server,
            http_server,
            stats,
        })
    }

    /// Serve QUIC/HTTP3 traffic
    pub async fn serve_quic(&self, addr: SocketAddr) -> Result<()> {
        info!("🚀 Starting QUIC/HTTP3 server on {}", addr);
        self.quic_server.serve(addr).await
    }

    /// Serve HTTP/1.1 fallback traffic
    pub async fn serve_http(&self, addr: SocketAddr) -> Result<()> {
        info!("🔄 Starting HTTP/1.1 fallback server on {}", addr);
        self.http_server.serve(addr).await
    }

    /// Get current proxy statistics
    pub async fn get_stats(&self) -> ProxyStats {
        let stats = self.stats.read().await;
        ProxyStats {
            active_connections: stats.active_connections,
            total_requests: stats.total_requests,
            quic_requests: stats.quic_requests,
            http_requests: stats.http_requests,
            bytes_transferred: stats.bytes_transferred,
            uptime_seconds: stats.uptime_seconds,
        }
    }

    /// Handle proxy request routing
    pub async fn route_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("🔀 Routing request: {} {}", req.method, req.path);

        // Update stats
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
            match req.protocol {
                Protocol::Quic => stats.quic_requests += 1,
                Protocol::Http => stats.http_requests += 1,
            }
        }

        // Route based on path
        match req.path.as_str() {
            path if path.starts_with("/api/containers") => {
                self.handle_container_request(req).await
            }
            path if path.starts_with("/api/images") => {
                self.handle_image_request(req).await
            }
            path if path.starts_with("/api/networks") => {
                self.handle_network_request(req).await
            }
            path if path.starts_with("/api/volumes") => {
                self.handle_volume_request(req).await
            }
            path if path.starts_with("/api/gaming") => {
                self.handle_gaming_request(req).await
            }
            path if path.starts_with("/api/system") => {
                self.handle_system_request(req).await
            }
            "/api/stats" => {
                self.handle_stats_request(req).await
            }
            _ => {
                self.handle_static_request(req).await
            }
        }
    }

    async fn handle_container_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("📦 Handling container request: {}", req.path);

        // Forward to Bolt API
        let bolt_url = format!("{}{}", self.config.bolt_api_url, req.path);

        // Use QUIC client if available, fallback to HTTP
        match self.forward_to_bolt_quic(&bolt_url, &req).await {
            Ok(response) => Ok(response),
            Err(e) => {
                debug!("QUIC forward failed, trying HTTP: {}", e);
                self.forward_to_bolt_http(&bolt_url, &req).await
            }
        }
    }

    async fn handle_image_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("🖼️ Handling image request: {}", req.path);
        let bolt_url = format!("{}{}", self.config.bolt_api_url, req.path);
        self.forward_to_bolt_quic(&bolt_url, &req).await
    }

    async fn handle_network_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("🌐 Handling network request: {}", req.path);
        let bolt_url = format!("{}{}", self.config.bolt_api_url, req.path);
        self.forward_to_bolt_quic(&bolt_url, &req).await
    }

    async fn handle_volume_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("💾 Handling volume request: {}", req.path);
        let bolt_url = format!("{}{}", self.config.bolt_api_url, req.path);
        self.forward_to_bolt_quic(&bolt_url, &req).await
    }

    async fn handle_gaming_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("🎮 Handling gaming request: {}", req.path);
        let bolt_url = format!("{}{}", self.config.bolt_api_url, req.path);
        self.forward_to_bolt_quic(&bolt_url, &req).await
    }

    async fn handle_system_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("⚙️ Handling system request: {}", req.path);

        // Some system requests go to agent service instead of Bolt
        if req.path.starts_with("/api/system/stats") {
            let agent_url = format!("http://localhost:{}{}", self.config.agent_port, req.path);
            return self.forward_to_agent(&agent_url, &req).await;
        }

        let bolt_url = format!("{}{}", self.config.bolt_api_url, req.path);
        self.forward_to_bolt_quic(&bolt_url, &req).await
    }

    async fn handle_stats_request(&self, _req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("📊 Handling stats request");

        let stats = self.get_stats().await;
        let response_body = serde_json::to_vec(&stats)?;

        Ok(ProxyResponse {
            status: 200,
            headers: vec![("content-type".to_string(), "application/json".to_string())],
            body: response_body,
        })
    }

    async fn handle_static_request(&self, req: ProxyRequest) -> Result<ProxyResponse> {
        debug!("📄 Handling static request: {}", req.path);

        // Serve Leptos frontend assets
        if req.path == "/" || req.path.starts_with("/assets") {
            // In production, these would be served from filesystem
            // For now, return a simple HTML page
            let html = include_str!("../../gpanel-web/index.html");
            return Ok(ProxyResponse {
                status: 200,
                headers: vec![("content-type".to_string(), "text/html".to_string())],
                body: html.as_bytes().to_vec(),
            });
        }

        // 404 for unknown paths
        Ok(ProxyResponse {
            status: 404,
            headers: vec![("content-type".to_string(), "text/plain".to_string())],
            body: b"Not Found".to_vec(),
        })
    }

    async fn forward_to_bolt_quic(&self, url: &str, _req: &ProxyRequest) -> Result<ProxyResponse> {
        debug!("⚡ Forwarding to Bolt via QUIC: {}", url);

        // TODO: Implement actual QUIC forwarding to Bolt
        // For now, return a mock response
        Ok(ProxyResponse {
            status: 200,
            headers: vec![("content-type".to_string(), "application/json".to_string())],
            body: br#"{"status": "forwarded_via_quic", "original_url": ""}"#.to_vec(),
        })
    }

    async fn forward_to_bolt_http(&self, url: &str, _req: &ProxyRequest) -> Result<ProxyResponse> {
        debug!("🔄 Forwarding to Bolt via HTTP: {}", url);

        // TODO: Implement HTTP forwarding to Bolt as fallback
        Ok(ProxyResponse {
            status: 200,
            headers: vec![("content-type".to_string(), "application/json".to_string())],
            body: br#"{"status": "forwarded_via_http", "original_url": ""}"#.to_vec(),
        })
    }

    async fn forward_to_agent(&self, url: &str, _req: &ProxyRequest) -> Result<ProxyResponse> {
        debug!("🔧 Forwarding to Agent: {}", url);

        // TODO: Implement forwarding to agent service
        Ok(ProxyResponse {
            status: 200,
            headers: vec![("content-type".to_string(), "application/json".to_string())],
            body: br#"{"status": "forwarded_to_agent", "original_url": ""}"#.to_vec(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProxyRequest {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub protocol: Protocol,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    Quic,
    Http,
}

#[derive(Debug, Clone)]
pub struct ProxyResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}