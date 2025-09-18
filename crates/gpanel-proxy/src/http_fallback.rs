use gpanel_core::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::proxy::ProxyStats;

pub struct HttpFallbackServer {
    // TODO: Implement HTTP fallback server
}

impl HttpFallbackServer {
    pub fn new(
        _config: gpanel_core::GhostPanelConfig,
        _stats: Arc<RwLock<ProxyStats>>,
    ) -> Result<Self> {
        Ok(Self {})
    }

    pub async fn serve(&self, _addr: SocketAddr) -> Result<()> {
        // TODO: Implement HTTP fallback server
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        Ok(())
    }
}