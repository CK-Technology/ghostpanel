use gpanel_core::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::proxy::ProxyStats;

pub struct QuicProxyServer {
    // TODO: Implement QUIC server
}

impl QuicProxyServer {
    pub async fn new(
        _config: gpanel_core::GhostPanelConfig,
        _dev_mode: bool,
        _max_connections: usize,
        _idle_timeout: u64,
        _stats: Arc<RwLock<ProxyStats>>,
    ) -> Result<Self> {
        Ok(Self {})
    }

    pub async fn serve(&self, _addr: SocketAddr) -> Result<()> {
        // TODO: Implement QUIC server
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        Ok(())
    }
}