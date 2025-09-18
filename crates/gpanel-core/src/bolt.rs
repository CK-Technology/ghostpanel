use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

use crate::container::*;

/// Bolt API client for container operations
#[derive(Debug, Clone)]
pub struct BoltClient {
    client: Client,
    base_url: String,
}

/// Bolt container API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoltResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Container operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerOperation {
    pub action: String,
    pub container_id: String,
    pub options: Option<HashMap<String, serde_json::Value>>,
}

/// Container logs request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerLogsRequest {
    pub container_id: String,
    pub follow: bool,
    pub tail: Option<u32>,
    pub timestamps: bool,
    pub since: Option<chrono::DateTime<chrono::Utc>>,
}

/// Container stats for real-time monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerStats {
    pub container_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cpu_percent: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub block_read: u64,
    pub block_write: u64,
    pub pid_count: u32,
}

/// System information from Bolt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoltSystemInfo {
    pub version: String,
    pub api_version: String,
    pub runtime: String,
    pub kernel_version: String,
    pub os: String,
    pub architecture: String,
    pub cpus: u32,
    pub memory_total: u64,
    pub storage_driver: String,
    pub containers_running: u32,
    pub containers_paused: u32,
    pub containers_stopped: u32,
    pub images_count: u32,
}

impl BoltClient {
    /// Create a new Bolt API client
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        Self {
            client,
            base_url: base_url.to_string(),
        }
    }

    /// Check if Bolt runtime is available
    pub async fn ping(&self) -> Result<bool> {
        let url = format!("{}/ping", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => {
                let success = response.status().is_success();
                if success {
                    debug!("Bolt runtime is available at {}", self.base_url);
                } else {
                    warn!("Bolt runtime returned error: {}", response.status());
                }
                Ok(success)
            }
            Err(e) => {
                warn!("Failed to ping Bolt runtime: {}", e);
                Ok(false)
            }
        }
    }

    /// Get system information from Bolt
    pub async fn system_info(&self) -> Result<BoltSystemInfo> {
        let url = format!("{}/system/info", self.base_url);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Bolt API error: {}", response.status()));
        }

        let bolt_response: BoltResponse<BoltSystemInfo> = response.json().await?;

        match bolt_response.data {
            Some(info) => Ok(info),
            None => Err(anyhow::anyhow!("No system info in response: {:?}", bolt_response.error)),
        }
    }

    /// List all containers
    pub async fn list_containers(&self, filter: Option<ContainerFilter>) -> Result<Vec<Container>> {
        let mut url = format!("{}/containers", self.base_url);

        // Add filter parameters if provided
        if let Some(filter) = filter {
            let mut params = Vec::new();

            if let Some(status) = filter.status {
                params.push(format!("status={}", serde_json::to_string(&status)?));
            }
            if let Some(name) = filter.name_contains {
                params.push(format!("name={}", urlencoding::encode(&name)));
            }
            if let Some(image) = filter.image_contains {
                params.push(format!("image={}", urlencoding::encode(&image)));
            }
            if let Some(gaming) = filter.has_gaming_config {
                params.push(format!("gaming={}", gaming));
            }
            if let Some(gpu) = filter.has_gpu {
                params.push(format!("gpu={}", gpu));
            }

            if !params.is_empty() {
                url.push('?');
                url.push_str(&params.join("&"));
            }
        }

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to list containers: {}", response.status()));
        }

        let bolt_response: BoltResponse<Vec<Container>> = response.json().await?;

        match bolt_response.data {
            Some(containers) => {
                info!("Retrieved {} containers from Bolt", containers.len());
                Ok(containers)
            }
            None => Err(anyhow::anyhow!("No containers in response: {:?}", bolt_response.error)),
        }
    }

    /// Get detailed container information
    pub async fn get_container(&self, id: &str) -> Result<Container> {
        let url = format!("{}/containers/{}", self.base_url, id);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Container not found: {}", id));
        }

        let bolt_response: BoltResponse<Container> = response.json().await?;

        match bolt_response.data {
            Some(container) => Ok(container),
            None => Err(anyhow::anyhow!("No container data: {:?}", bolt_response.error)),
        }
    }

    /// Start a container
    pub async fn start_container(&self, id: &str) -> Result<()> {
        self.container_operation(id, "start", None).await
    }

    /// Stop a container
    pub async fn stop_container(&self, id: &str, timeout: Option<u32>) -> Result<()> {
        let mut options = HashMap::new();
        if let Some(t) = timeout {
            options.insert("timeout".to_string(), serde_json::Value::Number(t.into()));
        }

        self.container_operation(id, "stop", Some(options)).await
    }

    /// Restart a container
    pub async fn restart_container(&self, id: &str, timeout: Option<u32>) -> Result<()> {
        let mut options = HashMap::new();
        if let Some(t) = timeout {
            options.insert("timeout".to_string(), serde_json::Value::Number(t.into()));
        }

        self.container_operation(id, "restart", Some(options)).await
    }

    /// Pause a container
    pub async fn pause_container(&self, id: &str) -> Result<()> {
        self.container_operation(id, "pause", None).await
    }

    /// Unpause a container
    pub async fn unpause_container(&self, id: &str) -> Result<()> {
        self.container_operation(id, "unpause", None).await
    }

    /// Kill a container
    pub async fn kill_container(&self, id: &str, signal: Option<&str>) -> Result<()> {
        let mut options = HashMap::new();
        if let Some(sig) = signal {
            options.insert("signal".to_string(), serde_json::Value::String(sig.to_string()));
        }

        self.container_operation(id, "kill", Some(options)).await
    }

    /// Remove a container
    pub async fn remove_container(&self, id: &str, force: bool, remove_volumes: bool) -> Result<()> {
        let mut options = HashMap::new();
        options.insert("force".to_string(), serde_json::Value::Bool(force));
        options.insert("volumes".to_string(), serde_json::Value::Bool(remove_volumes));

        self.container_operation(id, "remove", Some(options)).await
    }

    /// Create a new container
    pub async fn create_container(&self, request: CreateContainerRequest) -> Result<Container> {
        let url = format!("{}/containers", self.base_url);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to create container: {}", response.status()));
        }

        let bolt_response: BoltResponse<Container> = response.json().await?;

        match bolt_response.data {
            Some(container) => {
                info!("Created container: {} ({})", container.name, container.id);
                Ok(container)
            }
            None => Err(anyhow::anyhow!("No container data in create response: {:?}", bolt_response.error)),
        }
    }

    /// Get container logs
    pub async fn get_container_logs(&self, request: ContainerLogsRequest) -> Result<String> {
        let url = format!("{}/containers/{}/logs", self.base_url, request.container_id);

        let mut params = Vec::new();
        params.push(format!("follow={}", request.follow));
        params.push(format!("timestamps={}", request.timestamps));

        if let Some(tail) = request.tail {
            params.push(format!("tail={}", tail));
        }
        if let Some(since) = request.since {
            params.push(format!("since={}", since.timestamp()));
        }

        let url_with_params = format!("{}?{}", url, params.join("&"));

        let response = self.client.get(&url_with_params).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to get logs: {}", response.status()));
        }

        let logs = response.text().await?;
        Ok(logs)
    }

    /// Get container stats
    pub async fn get_container_stats(&self, id: &str) -> Result<ContainerStats> {
        let url = format!("{}/containers/{}/stats", self.base_url, id);

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to get stats: {}", response.status()));
        }

        let bolt_response: BoltResponse<ContainerStats> = response.json().await?;

        match bolt_response.data {
            Some(stats) => Ok(stats),
            None => Err(anyhow::anyhow!("No stats data: {:?}", bolt_response.error)),
        }
    }

    /// Execute a command in a container
    pub async fn exec_container(&self, id: &str, cmd: Vec<String>, interactive: bool) -> Result<String> {
        let url = format!("{}/containers/{}/exec", self.base_url, id);

        let request = serde_json::json!({
            "cmd": cmd,
            "interactive": interactive,
            "tty": interactive,
            "attach_stdout": true,
            "attach_stderr": true
        });

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to exec: {}", response.status()));
        }

        let output = response.text().await?;
        Ok(output)
    }

    /// Internal helper for container operations
    async fn container_operation(&self, id: &str, action: &str, options: Option<HashMap<String, serde_json::Value>>) -> Result<()> {
        let url = format!("{}/containers/{}/action", self.base_url, id);

        let operation = ContainerOperation {
            action: action.to_string(),
            container_id: id.to_string(),
            options,
        };

        let response = self.client
            .post(&url)
            .json(&operation)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Operation {} failed: {}", action, response.status()));
        }

        let bolt_response: BoltResponse<()> = response.json().await?;

        if !bolt_response.success {
            return Err(anyhow::anyhow!("Bolt operation failed: {:?}", bolt_response.error));
        }

        info!("Container {} operation {} completed", id, action);
        Ok(())
    }
}

/// Mock implementation for development/testing when Bolt is not available
pub struct MockBoltClient;

impl MockBoltClient {
    pub fn new() -> Self {
        Self
    }

    /// Generate mock containers for testing
    pub async fn list_containers(&self, _filter: Option<ContainerFilter>) -> Result<Vec<Container>> {
        let mock_containers = vec![
            Container {
                id: "mock_web_server_001".to_string(),
                name: "nginx-web".to_string(),
                image: "nginx:latest".to_string(),
                status: ContainerStatus::Running,
                ports: vec![
                    PortMapping {
                        container_port: 80,
                        host_port: Some(8080),
                        protocol: Protocol::Tcp,
                        host_ip: Some("0.0.0.0".to_string()),
                    }
                ],
                volumes: vec![],
                networks: vec!["bridge".to_string()],
                env: HashMap::new(),
                labels: HashMap::new(),
                created_at: chrono::Utc::now() - chrono::Duration::hours(2),
                started_at: Some(chrono::Utc::now() - chrono::Duration::hours(1)),
                finished_at: None,
                gaming_config: None,
                gpu_allocation: None,
                performance_metrics: Some(PerformanceMetrics {
                    cpu_usage: 15.2,
                    memory_usage: MemoryUsage {
                        used_mb: 128,
                        limit_mb: 512,
                        percentage: 25.0,
                    },
                    gpu_usage: None,
                    network_io: NetworkIo {
                        rx_bytes: 1024000,
                        tx_bytes: 2048000,
                        rx_packets: 1500,
                        tx_packets: 1200,
                    },
                    disk_io: DiskIo {
                        read_bytes: 512000,
                        write_bytes: 256000,
                        read_ops: 100,
                        write_ops: 50,
                    },
                    gaming_metrics: None,
                }),
            },
            Container {
                id: "mock_gaming_container_002".to_string(),
                name: "steam-gaming".to_string(),
                image: "gaming/steam-proton:latest".to_string(),
                status: ContainerStatus::Running,
                ports: vec![],
                volumes: vec![
                    VolumeMount {
                        source: "/home/user/games".to_string(),
                        target: "/games".to_string(),
                        read_only: false,
                        volume_type: VolumeType::Bind,
                    }
                ],
                networks: vec!["gaming".to_string()],
                env: HashMap::from([
                    ("STEAM_USER".to_string(), "player".to_string()),
                    ("PROTON_VERSION".to_string(), "8.0".to_string()),
                ]),
                labels: HashMap::from([
                    ("gaming".to_string(), "true".to_string()),
                    ("gpu".to_string(), "nvidia".to_string()),
                ]),
                created_at: chrono::Utc::now() - chrono::Duration::minutes(30),
                started_at: Some(chrono::Utc::now() - chrono::Duration::minutes(25)),
                finished_at: None,
                gaming_config: Some(GamingConfig {
                    proton_version: Some("8.0".to_string()),
                    wine_version: None,
                    steam_app_id: Some(440), // Team Fortress 2
                    optimization_profile: OptimizationProfile::Gaming,
                    audio_config: Some(AudioConfig {
                        system: AudioSystem::PipeWire,
                        latency: AudioLatency::Low,
                    }),
                }),
                gpu_allocation: Some(GpuAllocation {
                    device_id: "nvidia0".to_string(),
                    gpu_type: GpuType::Nvidia,
                    memory_mb: Some(8192),
                    compute_units: Some(4096),
                    isolation_level: IsolationLevel::Exclusive,
                }),
                performance_metrics: Some(PerformanceMetrics {
                    cpu_usage: 45.8,
                    memory_usage: MemoryUsage {
                        used_mb: 2048,
                        limit_mb: 4096,
                        percentage: 50.0,
                    },
                    gpu_usage: Some(GpuUsage {
                        utilization: 78.5,
                        memory_used_mb: 6144,
                        memory_total_mb: 8192,
                        temperature: Some(72.0),
                        power_usage: Some(180.0),
                    }),
                    network_io: NetworkIo {
                        rx_bytes: 10240000,
                        tx_bytes: 5120000,
                        rx_packets: 8000,
                        tx_packets: 6000,
                    },
                    disk_io: DiskIo {
                        read_bytes: 20480000,
                        write_bytes: 10240000,
                        read_ops: 2000,
                        write_ops: 1000,
                    },
                    gaming_metrics: Some(GamingMetrics {
                        fps: Some(144.0),
                        frame_time_ms: Some(6.9),
                        input_latency_ms: Some(12.5),
                        network_latency_ms: Some(25.0),
                        gpu_temperature: Some(72.0),
                    }),
                }),
            },
            Container {
                id: "mock_database_003".to_string(),
                name: "postgres-db".to_string(),
                image: "postgres:15".to_string(),
                status: ContainerStatus::Exited { code: 0 },
                ports: vec![
                    PortMapping {
                        container_port: 5432,
                        host_port: Some(5432),
                        protocol: Protocol::Tcp,
                        host_ip: Some("127.0.0.1".to_string()),
                    }
                ],
                volumes: vec![
                    VolumeMount {
                        source: "postgres_data".to_string(),
                        target: "/var/lib/postgresql/data".to_string(),
                        read_only: false,
                        volume_type: VolumeType::Volume,
                    }
                ],
                networks: vec!["database".to_string()],
                env: HashMap::from([
                    ("POSTGRES_DB".to_string(), "app".to_string()),
                    ("POSTGRES_USER".to_string(), "postgres".to_string()),
                ]),
                labels: HashMap::new(),
                created_at: chrono::Utc::now() - chrono::Duration::days(1),
                started_at: Some(chrono::Utc::now() - chrono::Duration::hours(12)),
                finished_at: Some(chrono::Utc::now() - chrono::Duration::minutes(10)),
                gaming_config: None,
                gpu_allocation: None,
                performance_metrics: None,
            },
        ];

        Ok(mock_containers)
    }

    pub async fn start_container(&self, _id: &str) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        Ok(())
    }

    pub async fn stop_container(&self, _id: &str, _timeout: Option<u32>) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        Ok(())
    }

    pub async fn restart_container(&self, _id: &str, _timeout: Option<u32>) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;
        Ok(())
    }

    pub async fn remove_container(&self, _id: &str, _force: bool, _remove_volumes: bool) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
        Ok(())
    }

    pub async fn get_container_logs(&self, _request: ContainerLogsRequest) -> Result<String> {
        let mock_logs = r#"2024-01-15 10:30:00 [INFO] Container started successfully
2024-01-15 10:30:01 [INFO] Initializing application
2024-01-15 10:30:02 [INFO] Loading configuration
2024-01-15 10:30:03 [INFO] Application ready to serve requests
2024-01-15 10:30:15 [INFO] Processed request: GET /health
2024-01-15 10:30:30 [INFO] Processed request: GET /api/status
2024-01-15 10:31:00 [INFO] System metrics: CPU 15.2%, Memory 128MB/512MB"#;

        Ok(mock_logs.to_string())
    }
}

impl Default for MockBoltClient {
    fn default() -> Self {
        Self::new()
    }
}