use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container information structure matching Bolt's container model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub networks: Vec<String>,
    pub env: HashMap<String, String>,
    pub labels: HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,

    // Gaming-specific fields
    pub gaming_config: Option<GamingConfig>,
    pub gpu_allocation: Option<GpuAllocation>,
    pub performance_metrics: Option<PerformanceMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerStatus {
    Created,
    Running,
    Paused,
    Restarting,
    Exited { code: i32 },
    Dead,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: u16,
    pub host_port: Option<u16>,
    pub protocol: Protocol,
    pub host_ip: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
    Quic, // QUIC protocol support
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
    pub volume_type: VolumeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VolumeType {
    Bind,
    Volume,
    Tmpfs,
}

/// Gaming-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingConfig {
    pub proton_version: Option<String>,
    pub wine_version: Option<String>,
    pub steam_app_id: Option<u32>,
    pub optimization_profile: OptimizationProfile,
    pub audio_config: Option<AudioConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationProfile {
    Gaming,
    Streaming,
    Competitive,
    Balanced,
    PowerSaving,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub system: AudioSystem,
    pub latency: AudioLatency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioSystem {
    PulseAudio,
    PipeWire,
    Alsa,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioLatency {
    UltraLow,
    Low,
    Normal,
    High,
}

/// GPU allocation for gaming containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuAllocation {
    pub device_id: String,
    pub gpu_type: GpuType,
    pub memory_mb: Option<u64>,
    pub compute_units: Option<u32>,
    pub isolation_level: IsolationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuType {
    Nvidia,
    Amd,
    Intel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IsolationLevel {
    Shared,
    Exclusive,
    Partitioned { partition_id: String },
}

/// Real-time performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: MemoryUsage,
    pub gpu_usage: Option<GpuUsage>,
    pub network_io: NetworkIo,
    pub disk_io: DiskIo,
    pub gaming_metrics: Option<GamingMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsage {
    pub used_mb: u64,
    pub limit_mb: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuUsage {
    pub utilization: f64,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub temperature: Option<f32>,
    pub power_usage: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIo {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIo {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_ops: u64,
    pub write_ops: u64,
}

/// Gaming-specific performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingMetrics {
    pub fps: Option<f32>,
    pub frame_time_ms: Option<f32>,
    pub input_latency_ms: Option<f32>,
    pub network_latency_ms: Option<f32>,
    pub gpu_temperature: Option<f32>,
}

/// Container creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContainerRequest {
    pub name: Option<String>,
    pub image: String,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub networks: Vec<String>,
    pub env: HashMap<String, String>,
    pub labels: HashMap<String, String>,
    pub gaming_config: Option<GamingConfig>,
    pub gpu_allocation: Option<GpuAllocation>,
    pub restart_policy: RestartPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure { max_retries: Option<u32> },
    UnlessStopped,
}

/// Container list filters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerFilter {
    pub status: Option<ContainerStatus>,
    pub name_contains: Option<String>,
    pub image_contains: Option<String>,
    pub has_gaming_config: Option<bool>,
    pub has_gpu: Option<bool>,
    pub network: Option<String>,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        RestartPolicy::No
    }
}