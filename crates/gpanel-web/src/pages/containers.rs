use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use crate::pages::registries::{RegistryConfig, ImageInfo};

/// Container status enum for UI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerStatus {
    Created,
    Running,
    Paused,
    Restarting,
    Exited { code: i32 },
    Dead,
    Unknown,
}

impl std::fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerStatus::Created => write!(f, "Created"),
            ContainerStatus::Running => write!(f, "Running"),
            ContainerStatus::Paused => write!(f, "Paused"),
            ContainerStatus::Restarting => write!(f, "Restarting"),
            ContainerStatus::Exited { code } => write!(f, "Exited ({})", code),
            ContainerStatus::Dead => write!(f, "Dead"),
            ContainerStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Port mapping for containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: u16,
    pub host_port: Option<u16>,
    pub protocol: String,
    pub host_ip: Option<String>,
}

/// Volume mount for containers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
    pub volume_type: String,
}

/// Gaming configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingConfig {
    pub proton_version: Option<String>,
    pub wine_version: Option<String>,
    pub steam_app_id: Option<u32>,
    pub optimization_profile: String,
}

/// GPU allocation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuAllocation {
    pub device_id: String,
    pub gpu_type: String,
    pub memory_mb: Option<u64>,
    pub compute_units: Option<u32>,
    pub isolation_level: String,
}

/// Performance metrics
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamingMetrics {
    pub fps: Option<f32>,
    pub frame_time_ms: Option<f32>,
    pub input_latency_ms: Option<f32>,
    pub network_latency_ms: Option<f32>,
    pub gpu_temperature: Option<f32>,
}

/// Container model for UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: ContainerStatus,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub networks: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
    pub labels: std::collections::HashMap<String, String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    pub gaming_config: Option<GamingConfig>,
    pub gpu_allocation: Option<GpuAllocation>,
    pub performance_metrics: Option<PerformanceMetrics>,
}

/// Container list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerListResponse {
    pub containers: Vec<Container>,
}

/// Container operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerOperationRequest {
    pub action: String,
    pub timeout: Option<u32>,
    pub force: Option<bool>,
    pub remove_volumes: Option<bool>,
}

/// Container creation request (matches gpanel-core structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerCreateRequest {
    pub name: Option<String>,
    pub image: String,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub networks: Vec<String>,
    pub env: std::collections::HashMap<String, String>,
    pub labels: std::collections::HashMap<String, String>,
    pub gaming_config: Option<GamingConfig>,
    pub gpu_allocation: Option<GpuAllocation>,
    pub restart_policy: RestartPolicy,
}

/// Restart policy enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    No,
    Always,
    UnlessStopped,
    OnFailure,
}

/// Operation result response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}

/// Format file size in human readable format
fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Format uptime duration
fn format_uptime(started_at: Option<chrono::DateTime<chrono::Utc>>) -> String {
    match started_at {
        Some(start) => {
            let duration = chrono::Utc::now().signed_duration_since(start);
            let days = duration.num_days();
            let hours = duration.num_hours() % 24;
            let minutes = duration.num_minutes() % 60;

            if days > 0 {
                format!("{}d {}h {}m", days, hours, minutes)
            } else if hours > 0 {
                format!("{}h {}m", hours, minutes)
            } else {
                format!("{}m", minutes)
            }
        }
        None => "Not started".to_string(),
    }
}

#[component]
pub fn ContainerList() -> impl IntoView {
    let (containers, set_containers) = create_signal(Vec::<Container>::new());
    let (loading, set_loading) = create_signal(true);
    let (error_message, set_error_message) = create_signal(None::<String>);
    let (selected_container, set_selected_container) = create_signal(None::<Container>);
    let (show_logs, set_show_logs) = create_signal(false);
    let (container_logs, set_container_logs) = create_signal(String::new());
    let (show_create_wizard, set_show_create_wizard) = create_signal(false);

    // Load containers on mount
    create_effect(move |_| {
        spawn_local(async move {
            load_containers(set_containers, set_loading, set_error_message).await;
        });
    });

    // Auto-refresh every 5 seconds
    create_effect(move |_| {
        let interval = set_interval(
            move || {
                spawn_local(async move {
                    load_containers(set_containers, set_loading, set_error_message).await;
                });
            },
            std::time::Duration::from_secs(5),
        );

        on_cleanup(move || {
            clear_interval(interval);
        });
    });

    let container_operation = move |container_id: String, action: String| {
        spawn_local(async move {
            set_loading.set(true);

            let request = ContainerOperationRequest {
                action: action.clone(),
                timeout: Some(30),
                force: None,
                remove_volumes: None,
            };

            let url = match action.as_str() {
                "start" => format!("http://localhost:8000/api/v1/containers/{}/start", container_id),
                "stop" => format!("http://localhost:8000/api/v1/containers/{}/stop", container_id),
                "restart" => format!("http://localhost:8000/api/v1/containers/{}/restart", container_id),
                _ => {
                    set_error_message.set(Some(format!("Unknown action: {}", action)));
                    set_loading.set(false);
                    return;
                }
            };

            match Request::post(&url)
                .json(&request)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if let Ok(result) = response.json::<OperationResult>().await {
                        if result.success {
                            set_error_message.set(Some(format!("✅ {}", result.message)));
                            // Refresh container list
                            load_containers(set_containers, set_loading, set_error_message).await;
                        } else {
                            set_error_message.set(Some(format!("❌ {}", result.message)));
                        }
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("❌ Operation failed: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    let show_container_logs = move |container: Container| {
        spawn_local(async move {
            set_selected_container.set(Some(container.clone()));
            set_show_logs.set(true);

            let url = format!("http://localhost:8000/api/v1/containers/{}/logs", container.id);

            match Request::get(&url).send().await {
                Ok(response) => {
                    if let Ok(logs) = response.text().await {
                        set_container_logs.set(logs);
                    } else {
                        set_container_logs.set("Failed to load logs".to_string());
                    }
                }
                Err(e) => {
                    set_container_logs.set(format!("Error loading logs: {}", e));
                }
            }
        });
    };

    view! {
        <div class="container-list">
            <div class="header-section">
                <h2>"Containers"</h2>
                <p>"Manage your Bolt containers with advanced monitoring and gaming features"</p>
                <div style="display: flex; gap: 10px;">
                    <button
                        class="btn-primary"
                        on:click=move |_| {
                            set_show_create_wizard.set(true);
                        }
                    >
                        "Create Container"
                    </button>
                    <button
                        class="btn-primary"
                        style="background-color: #6c757d;"
                        on:click=move |_| {
                            spawn_local(async move {
                                load_containers(set_containers, set_loading, set_error_message).await;
                            });
                        }
                    >
                        "Refresh"
                    </button>
                </div>
            </div>

            // Error/Success message display
            {move || {
                if let Some(message) = error_message.get() {
                    let is_success = message.starts_with("✅");
                    view! {
                        <div
                            class="message-banner"
                            style=format!(
                                "background-color: {}; color: white; padding: 10px; border-radius: 4px; margin-bottom: 20px;",
                                if is_success { "#27ae60" } else { "#e74c3c" }
                            )
                        >
                            {message}
                            <button style="float: right; background: none; border: none; color: white; cursor: pointer;"
                                    on:click=move |_| set_error_message.set(None)>
                                "×"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Loading indicator
            {move || {
                if loading.get() {
                    view! {
                        <div style="text-align: center; padding: 20px;">
                            <div style="display: inline-block; width: 20px; height: 20px; border: 3px solid #f3f3f3; border-top: 3px solid #3498db; border-radius: 50%; animation: spin 1s linear infinite;"></div>
                            <span style="margin-left: 10px;">"Loading containers..."</span>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Container grid
            <div class="container-grid" style="display: grid; gap: 20px;">
                <For
                    each=move || containers.get()
                    key=|container| container.id.clone()
                    children=move |container| {
                        let container_for_start = container.clone();
                        let container_for_stop = container.clone();
                        let container_for_restart = container.clone();
                        let container_for_logs = container.clone();

                        view! {
                            <div class="container-card" style="background-color: #2c3e50; border-radius: 8px; padding: 20px; border: 1px solid #4a5568;">
                                // Container header
                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                                    <div>
                                        <h3 style="margin: 0; color: #3498db; display: flex; align-items: center; gap: 10px;">
                                            {&container.name}
                                            <span class=format!(
                                                "status-badge status-{}",
                                                match container.status {
                                                    ContainerStatus::Running => "running",
                                                    ContainerStatus::Exited { .. } => "stopped",
                                                    ContainerStatus::Paused => "paused",
                                                    _ => "unknown"
                                                }
                                            )>
                                                {container.status.to_string()}
                                            </span>
                                            {container.gaming_config.as_ref().map(|_| view! {
                                                <span class="gaming-badge">"GAMING"</span>
                                            })}
                                            {container.gpu_allocation.as_ref().map(|_| view! {
                                                <span class="gpu-indicator">"GPU"</span>
                                            })}
                                        </h3>
                                        <p style="margin: 5px 0; color: #bbb; font-size: 14px;">{&container.image}</p>
                                    </div>
                                    <div style="font-size: 12px; color: #888; text-align: right;">
                                        <div>
                                            <strong>"ID: "</strong>
                                            <code style="background-color: #1a1a1a; padding: 2px 4px; border-radius: 2px;">
                                                {&container.id[..12]}
                                            </code>
                                        </div>
                                        <div style="margin-top: 4px;">
                                            <strong>"Uptime: "</strong>
                                            {format_uptime(container.started_at)}
                                        </div>
                                    </div>
                                </div>

                                // Container details
                                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 15px; margin-bottom: 15px;">
                                    <div>
                                        <h4 style="margin: 0 0 8px 0; color: #fff; font-size: 14px;">"Configuration"</h4>

                                        // Ports
                                        {if !container.ports.is_empty() {
                                            view! {
                                                <div style="margin-bottom: 8px;">
                                                    <strong>"Ports: "</strong>
                                                    <span style="color: #3498db;">
                                                        {container.ports.iter().map(|port| {
                                                            format!("{}:{}",
                                                                port.host_port.map(|p| p.to_string()).unwrap_or_else(|| "auto".to_string()),
                                                                port.container_port)
                                                        }).collect::<Vec<_>>().join(", ")}
                                                    </span>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }}

                                        // Networks
                                        {if !container.networks.is_empty() {
                                            view! {
                                                <div style="margin-bottom: 8px;">
                                                    <strong>"Networks: "</strong>
                                                    <span style="color: #27ae60;">
                                                        {container.networks.join(", ")}
                                                    </span>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }}

                                        // Gaming config
                                        {if let Some(gaming) = &container.gaming_config {
                                            view! {
                                                <div style="margin-bottom: 8px;">
                                                    <strong>"Proton: "</strong>
                                                    <span style="color: #9b59b6;">
                                                        {gaming.proton_version.as_ref().unwrap_or(&"None".to_string())}
                                                    </span>
                                                    {gaming.steam_app_id.map(|id| view! {
                                                        <div>
                                                            <strong>"Steam App: "</strong>
                                                            <span style="color: #9b59b6;">{id.to_string()}</span>
                                                        </div>
                                                    })}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }}
                                    </div>

                                    <div>
                                        <h4 style="margin: 0 0 8px 0; color: #fff; font-size: 14px;">"Performance"</h4>

                                        {if let Some(metrics) = &container.performance_metrics {
                                            view! {
                                                <div>
                                                    <div style="margin-bottom: 6px;">
                                                        <strong>"CPU: "</strong>
                                                        <span style="color: #f39c12;">{format!("{:.1}%", metrics.cpu_usage)}</span>
                                                    </div>
                                                    <div style="margin-bottom: 6px;">
                                                        <strong>"Memory: "</strong>
                                                        <span style="color: #e74c3c;">
                                                            {format_size(metrics.memory_usage.used_mb * 1024 * 1024)}
                                                            " / "
                                                            {format_size(metrics.memory_usage.limit_mb * 1024 * 1024)}
                                                            " ("
                                                            {format!("{:.1}%", metrics.memory_usage.percentage)}
                                                            ")"
                                                        </span>
                                                    </div>
                                                    {if let Some(gpu) = &metrics.gpu_usage {
                                                        view! {
                                                            <div style="margin-bottom: 6px;">
                                                                <strong>"GPU: "</strong>
                                                                <span style="color: #f39c12;">{format!("{:.1}%", gpu.utilization)}</span>
                                                                <div style="font-size: 12px; color: #888;">
                                                                    {format_size(gpu.memory_used_mb * 1024 * 1024)}
                                                                    " / "
                                                                    {format_size(gpu.memory_total_mb * 1024 * 1024)}
                                                                </div>
                                                            </div>
                                                        }.into_view()
                                                    } else {
                                                        view! { <div></div> }.into_view()
                                                    }}
                                                    {if let Some(gaming_metrics) = &metrics.gaming_metrics {
                                                        view! {
                                                            <div style="margin-bottom: 6px;">
                                                                <strong>"FPS: "</strong>
                                                                <span style="color: #2ecc71;">
                                                                    {gaming_metrics.fps.map(|f| format!("{:.0}", f)).unwrap_or_else(|| "N/A".to_string())}
                                                                </span>
                                                            </div>
                                                        }.into_view()
                                                    } else {
                                                        view! { <div></div> }.into_view()
                                                    }}
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! {
                                                <div style="color: #888; font-style: italic;">"No metrics available"</div>
                                            }.into_view()
                                        }}
                                    </div>
                                </div>

                                // Container actions
                                <div style="display: flex; gap: 8px; align-items: center;">
                                    {match container.status {
                                        ContainerStatus::Running => view! {
                                            <button
                                                class="btn-danger"
                                                style="padding: 6px 12px; font-size: 12px;"
                                                on:click=move |_| container_operation(container_for_stop.id.clone(), "stop".to_string())
                                                disabled=move || loading.get()
                                            >
                                                "Stop"
                                            </button>
                                            <button
                                                class="btn-primary"
                                                style="padding: 6px 12px; font-size: 12px;"
                                                on:click=move |_| container_operation(container_for_restart.id.clone(), "restart".to_string())
                                                disabled=move || loading.get()
                                            >
                                                "Restart"
                                            </button>
                                        }.into_view(),
                                        _ => view! {
                                            <button
                                                class="btn-success"
                                                style="padding: 6px 12px; font-size: 12px;"
                                                on:click=move |_| container_operation(container_for_start.id.clone(), "start".to_string())
                                                disabled=move || loading.get()
                                            >
                                                "Start"
                                            </button>
                                        }.into_view()
                                    }}

                                    <button
                                        class="btn-primary"
                                        style="padding: 6px 12px; font-size: 12px; background-color: #6c757d;"
                                        on:click=move |_| show_container_logs(container_for_logs.clone())
                                    >
                                        "Logs"
                                    </button>

                                    <button
                                        class="btn-primary"
                                        style="padding: 6px 12px; font-size: 12px; background-color: #17a2b8;"
                                        on:click=move |_| {
                                            // TODO: Navigate to container details
                                            web_sys::console::log_1(&format!("View details for {}", container.id).into());
                                        }
                                    >
                                        "Details"
                                    </button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            // Container logs modal
            {move || {
                if show_logs.get() {
                    if let Some(container) = selected_container.get() {
                        view! {
                            <div style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;">
                                <div class="container-card" style="width: 80%; max-width: 800px; height: 60%; max-height: 600px; display: flex; flex-direction: column;">
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px;">
                                        <h3 style="margin: 0;">"Logs: " {&container.name}</h3>
                                        <button
                                            style="background: none; border: none; color: white; font-size: 24px; cursor: pointer;"
                                            on:click=move |_| set_show_logs.set(false)
                                        >
                                            "×"
                                        </button>
                                    </div>
                                    <div style="flex: 1; background-color: #1a1a1a; border-radius: 4px; padding: 15px; overflow-y: auto; font-family: 'Courier New', monospace; font-size: 12px; white-space: pre-wrap;">
                                        {container_logs.get()}
                                    </div>
                                </div>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Container creation wizard modal
            {move || {
                if show_create_wizard.get() {
                    view! {
                        <ContainerCreateWizard
                            show=show_create_wizard
                            on_close=move || set_show_create_wizard.set(false)
                            on_created=move || {
                                set_show_create_wizard.set(false);
                                spawn_local(async move {
                                    load_containers(set_containers, set_loading, set_error_message).await;
                                });
                            }
                        />
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Summary stats
            <div class="container-card" style="margin-top: 30px;">
                <h3>"Summary"</h3>
                <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 20px; margin-top: 15px;">
                    {move || {
                        let containers_list = containers.get();
                        let total = containers_list.len();
                        let running = containers_list.iter().filter(|c| matches!(c.status, ContainerStatus::Running)).count();
                        let stopped = containers_list.iter().filter(|c| matches!(c.status, ContainerStatus::Exited { .. })).count();
                        let gaming = containers_list.iter().filter(|c| c.gaming_config.is_some()).count();
                        let gpu_enabled = containers_list.iter().filter(|c| c.gpu_allocation.is_some()).count();

                        view! {
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: bold; color: #3498db;">{total}</div>
                                <div style="color: #bbb; font-size: 14px;">"Total Containers"</div>
                            </div>
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: bold; color: #2ecc71;">{running}</div>
                                <div style="color: #bbb; font-size: 14px;">"Running"</div>
                            </div>
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: bold; color: #e74c3c;">{stopped}</div>
                                <div style="color: #bbb; font-size: 14px;">"Stopped"</div>
                            </div>
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: bold; color: #9b59b6;">{gaming}</div>
                                <div style="color: #bbb; font-size: 14px;">"Gaming"</div>
                            </div>
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: bold; color: #f39c12;">{gpu_enabled}</div>
                                <div style="color: #bbb; font-size: 14px;">"GPU Enabled"</div>
                            </div>
                        }
                    }}
                </div>
            </div>
        </div>
    }
}

/// Load containers from API
async fn load_containers(
    set_containers: WriteSignal<Vec<Container>>,
    set_loading: WriteSignal<bool>,
    set_error_message: WriteSignal<Option<String>>,
) {
    match Request::get("http://localhost:8000/api/v1/containers")
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(container_list) = response.json::<ContainerListResponse>().await {
                set_containers.set(container_list.containers);
                set_error_message.set(None);
            } else {
                set_error_message.set(Some("Failed to parse container data".to_string()));
            }
        }
        Err(e) => {
            set_error_message.set(Some(format!("Failed to load containers: {}", e)));
        }
    }
    set_loading.set(false);
}

/// Mock function for setInterval (would be provided by web framework)
fn set_interval<F>(f: F, duration: std::time::Duration) -> i32
where F: Fn() + 'static
{
    // This is a placeholder - in real implementation would use web_sys::setInterval
    0
}

/// Mock function for clearInterval
fn clear_interval(_id: i32) {
    // This is a placeholder - in real implementation would use web_sys::clearInterval
}

#[component]
pub fn ContainerCreateWizard<F1, F2>(
    show: ReadSignal<bool>,
    on_close: F1,
    on_created: F2,
) -> impl IntoView
where
    F1: Fn() + 'static + Clone,
    F2: Fn() + 'static + Clone,
{
    let (current_step, set_current_step) = create_signal(1);
    let (container_name, set_container_name) = create_signal(String::new());
    let (selected_image, set_selected_image) = create_signal(None::<ImageInfo>);
    let (search_query, set_search_query) = create_signal(String::new());
    let (search_results, set_search_results) = create_signal(Vec::<ImageInfo>::new());
    let (registries, set_registries) = create_signal(Vec::<RegistryConfig>::new());
    let (loading, set_loading) = create_signal(false);
    let (error_message, set_error_message) = create_signal(None::<String>);

    // Container configuration
    let (ports, set_ports) = create_signal(Vec::<PortMapping>::new());
    let (volumes, set_volumes) = create_signal(Vec::<VolumeMount>::new());
    let (env_vars, set_env_vars) = create_signal(std::collections::HashMap::<String, String>::new());
    let (networks, set_networks) = create_signal(vec!["bridge".to_string()]);
    let (enable_gaming, set_enable_gaming) = create_signal(false);
    let (enable_gpu, set_enable_gpu) = create_signal(false);
    let (restart_policy, set_restart_policy) = create_signal(RestartPolicy::No);

    // Load registries on mount
    create_effect(move |_| {
        if show.get() {
            spawn_local(async move {
                load_registries_for_wizard(set_registries).await;
            });
        }
    });

    let search_images = move || {
        let query = search_query.get();
        if query.is_empty() {
            return;
        }

        spawn_local(async move {
            set_loading.set(true);
            set_error_message.set(None);

            match Request::get(&format!("http://localhost:8000/api/v1/images/search?q={}", query))
                .send()
                .await
            {
                Ok(response) => {
                    if let Ok(images) = response.json::<Vec<ImageInfo>>().await {
                        set_search_results.set(images);
                    } else {
                        set_error_message.set(Some("Failed to parse search results".to_string()));
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Search failed: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    let add_port = move || {
        let mut current_ports = ports.get();
        current_ports.push(PortMapping {
            container_port: 80,
            host_port: None,
            protocol: "tcp".to_string(),
            host_ip: None,
        });
        set_ports.set(current_ports);
    };

    let add_volume = move || {
        let mut current_volumes = volumes.get();
        current_volumes.push(VolumeMount {
            source: "/host/path".to_string(),
            target: "/container/path".to_string(),
            read_only: false,
            volume_type: "bind".to_string(),
        });
        set_volumes.set(current_volumes);
    };

    let add_env_var = move || {
        let mut current_env = env_vars.get();
        current_env.insert("NEW_VAR".to_string(), "value".to_string());
        set_env_vars.set(current_env);
    };

    let create_container = move || {
        let name = container_name.get();
        let image = match selected_image.get() {
            Some(img) => format!("{}:{}", img.name, img.tag),
            None => {
                set_error_message.set(Some("Please select an image".to_string()));
                return;
            }
        };

        if name.is_empty() {
            set_error_message.set(Some("Please enter a container name".to_string()));
            return;
        }

        let gaming_config = if enable_gaming.get() {
            Some(GamingConfig {
                proton_version: Some("8.0-3".to_string()),
                wine_version: None,
                steam_app_id: None,
                optimization_profile: "gaming".to_string(),
            })
        } else {
            None
        };

        let gpu_allocation = if enable_gpu.get() {
            Some(GpuAllocation {
                device_id: "gpu0".to_string(),
                gpu_type: "nvidia".to_string(),
                memory_mb: Some(2048),
                compute_units: Some(1),
                isolation_level: "process".to_string(),
            })
        } else {
            None
        };

        let request = ContainerCreateRequest {
            name: Some(name),
            image,
            ports: ports.get(),
            volumes: volumes.get(),
            networks: networks.get(),
            env: env_vars.get(),
            labels: std::collections::HashMap::new(),
            gaming_config,
            gpu_allocation,
            restart_policy: restart_policy.get(),
        };

        spawn_local(async move {
            set_loading.set(true);
            set_error_message.set(None);

            match Request::post("http://localhost:8000/api/v1/containers")
                .json(&request)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if response.status() == 201 {
                        on_created();
                    } else {
                        set_error_message.set(Some("Failed to create container".to_string()));
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Creation failed: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    let on_close_clone = on_close.clone();

    view! {
        <div style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0,0,0,0.7); z-index: 3000; display: flex; align-items: center; justify-content: center;">
            <div class="container-card" style="width: 90%; max-width: 1000px; height: 80%; max-height: 700px; display: flex; flex-direction: column;">
                // Header
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px; border-bottom: 1px solid #4a5568; padding-bottom: 15px;">
                    <h2 style="margin: 0; color: #3498db;">"Create Container"</h2>
                    <button
                        style="background: none; border: none; color: white; font-size: 24px; cursor: pointer;"
                        on:click=move |_| on_close_clone()
                    >
                        "×"
                    </button>
                </div>

                // Step indicator
                <div style="display: flex; justify-content: center; margin-bottom: 30px;">
                    <div style="display: flex; align-items: center; gap: 20px;">
                        {(1..=4).map(|step| {
                            let is_active = move || current_step.get() == step;
                            let is_completed = move || current_step.get() > step;
                            view! {
                                <div style="display: flex; align-items: center; gap: 10px;">
                                    <div class=format!("step-indicator step-{}", step)
                                         style=move || format!(
                                            "width: 30px; height: 30px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-weight: bold; {}",
                                            if is_completed() {
                                                "background-color: #2ecc71; color: white;"
                                            } else if is_active() {
                                                "background-color: #3498db; color: white;"
                                            } else {
                                                "background-color: #4a5568; color: #bbb;"
                                            }
                                        )>
                                        {if is_completed() { "✓" } else { &step.to_string() }}
                                    </div>
                                    {if step < 4 {
                                        view! {
                                            <div style="width: 40px; height: 2px; background-color: #4a5568;"></div>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                // Error message
                {move || {
                    if let Some(message) = error_message.get() {
                        view! {
                            <div style="background-color: #e74c3c; color: white; padding: 10px; border-radius: 4px; margin-bottom: 20px;">
                                {message}
                                <button style="float: right; background: none; border: none; color: white; cursor: pointer;"
                                        on:click=move |_| set_error_message.set(None)>
                                    "×"
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}

                // Step content
                <div style="flex: 1; overflow-y: auto;">
                    {move || {
                        match current_step.get() {
                            1 => view! {
                                <div class="wizard-step">
                                    <h3>"Step 1: Select Image"</h3>
                                    <p>"Choose a container image from your registries"</p>

                                    <div style="margin-bottom: 20px;">
                                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Search Images:"</label>
                                        <div style="display: flex; gap: 10px;">
                                            <input
                                                type="text"
                                                placeholder="Search for images..."
                                                style="flex: 1; padding: 8px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                prop:value=move || search_query.get()
                                                on:input=move |ev| set_search_query.set(event_target_value(&ev))
                                                on:keydown=move |ev| {
                                                    if ev.key() == "Enter" {
                                                        search_images();
                                                    }
                                                }
                                            />
                                            <button
                                                class="btn-primary"
                                                on:click=move |_| search_images()
                                                disabled=move || loading.get()
                                            >
                                                "Search"
                                            </button>
                                        </div>
                                    </div>

                                    {if loading.get() {
                                        view! {
                                            <div style="text-align: center; padding: 20px;">
                                                "Searching images..."
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="image-results" style="max-height: 300px; overflow-y: auto;">
                                                <For
                                                    each=move || search_results.get()
                                                    key=|image| format!("{}:{}", image.name, image.tag)
                                                    children=move |image| {
                                                        let image_for_select = image.clone();
                                                        let is_selected = move || {
                                                            if let Some(selected) = selected_image.get() {
                                                                selected.name == image.name && selected.tag == image.tag
                                                            } else {
                                                                false
                                                            }
                                                        };

                                                        view! {
                                                            <div
                                                                class="image-item"
                                                                style=move || format!(
                                                                    "padding: 10px; border: 1px solid {}; border-radius: 4px; margin-bottom: 10px; cursor: pointer; background-color: {};",
                                                                    if is_selected() { "#3498db" } else { "#4a5568" },
                                                                    if is_selected() { "#34495e" } else { "transparent" }
                                                                )
                                                                on:click=move |_| {
                                                                    set_selected_image.set(Some(image_for_select.clone()));
                                                                }
                                                            >
                                                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                                                    <div>
                                                                        <div style="font-weight: bold; color: #3498db;">
                                                                            {&image.name}
                                                                            <span style="color: #f39c12; margin-left: 5px;">":"</span>
                                                                            <span style="color: #2ecc71;">{&image.tag}</span>
                                                                        </div>
                                                                        <div style="font-size: 12px; color: #bbb; margin-top: 2px;">
                                                                            {&image.registry_url}
                                                                        </div>
                                                                    </div>
                                                                    <div style="text-align: right; font-size: 12px; color: #888;">
                                                                        {if let Some(size) = image.size {
                                                                            format_size(size)
                                                                        } else {
                                                                            "Unknown size".to_string()
                                                                        }}
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        }
                                                    }
                                                />
                                            </div>
                                        }.into_view()
                                    }}
                                </div>
                            }.into_view(),
                            2 => view! {
                                <div class="wizard-step">
                                    <h3>"Step 2: Basic Configuration"</h3>
                                    <p>"Configure basic container settings"</p>

                                    <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px;">
                                        <div>
                                            <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Container Name:"</label>
                                            <input
                                                type="text"
                                                placeholder="my-container"
                                                style="width: 100%; padding: 8px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                prop:value=move || container_name.get()
                                                on:input=move |ev| set_container_name.set(event_target_value(&ev))
                                            />
                                        </div>

                                        <div>
                                            <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Restart Policy:"</label>
                                            <select
                                                style="width: 100%; padding: 8px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                on:change=move |ev| {
                                                    let policy = match event_target_value(&ev).as_str() {
                                                        "always" => RestartPolicy::Always,
                                                        "unless-stopped" => RestartPolicy::UnlessStopped,
                                                        "on-failure" => RestartPolicy::OnFailure,
                                                        _ => RestartPolicy::No,
                                                    };
                                                    set_restart_policy.set(policy);
                                                }
                                            >
                                                <option value="no">"No"</option>
                                                <option value="always">"Always"</option>
                                                <option value="unless-stopped">"Unless Stopped"</option>
                                                <option value="on-failure">"On Failure"</option>
                                            </select>
                                        </div>
                                    </div>

                                    <div style="margin-top: 20px;">
                                        <h4>"Selected Image:"</h4>
                                        {move || {
                                            if let Some(image) = selected_image.get() {
                                                view! {
                                                    <div style="background-color: #34495e; padding: 10px; border-radius: 4px;">
                                                        <span style="color: #3498db; font-weight: bold;">{&image.name}</span>
                                                        <span style="color: #f39c12;">":"</span>
                                                        <span style="color: #2ecc71;">{&image.tag}</span>
                                                        <div style="font-size: 12px; color: #bbb; margin-top: 5px;">
                                                            {&image.registry_url}
                                                        </div>
                                                    </div>
                                                }.into_view()
                                            } else {
                                                view! {
                                                    <div style="color: #e74c3c; font-style: italic;">
                                                        "No image selected"
                                                    </div>
                                                }.into_view()
                                            }
                                        }}
                                    </div>

                                    <div style="margin-top: 20px;">
                                        <h4>"Special Features:"</h4>
                                        <div style="display: flex; gap: 20px;">
                                            <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || enable_gaming.get()
                                                    on:change=move |ev| set_enable_gaming.set(event_target_checked(&ev))
                                                />
                                                <span style="color: #9b59b6; font-weight: bold;">"Gaming Mode"</span>
                                            </label>
                                            <label style="display: flex; align-items: center; gap: 5px; cursor: pointer;">
                                                <input
                                                    type="checkbox"
                                                    prop:checked=move || enable_gpu.get()
                                                    on:change=move |ev| set_enable_gpu.set(event_target_checked(&ev))
                                                />
                                                <span style="color: #f39c12; font-weight: bold;">"GPU Access"</span>
                                            </label>
                                        </div>
                                    </div>
                                </div>
                            }.into_view(),
                            3 => view! {
                                <div class="wizard-step">
                                    <h3>"Step 3: Network & Storage"</h3>
                                    <p>"Configure ports, volumes, and environment variables"</p>

                                    // Port mappings
                                    <div style="margin-bottom: 30px;">
                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;">
                                            <h4 style="margin: 0;">"Port Mappings:"</h4>
                                            <button class="btn-primary" style="padding: 5px 10px; font-size: 12px;" on:click=move |_| add_port()>
                                                "Add Port"
                                            </button>
                                        </div>
                                        <For
                                            each=move || ports.get().into_iter().enumerate().collect::<Vec<_>>()
                                            key=|(i, _)| *i
                                            children=move |(index, port)| {
                                                view! {
                                                    <div style="display: grid; grid-template-columns: 1fr 1fr 1fr auto; gap: 10px; margin-bottom: 10px; align-items: end;">
                                                        <div>
                                                            <label style="display: block; margin-bottom: 5px; font-size: 12px;">"Host Port:"</label>
                                                            <input
                                                                type="number"
                                                                placeholder="Auto"
                                                                style="width: 100%; padding: 6px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                                prop:value=move || port.host_port.map(|p| p.to_string()).unwrap_or_default()
                                                                on:input=move |ev| {
                                                                    let mut current_ports = ports.get();
                                                                    let value = event_target_value(&ev);
                                                                    current_ports[index].host_port = if value.is_empty() {
                                                                        None
                                                                    } else {
                                                                        value.parse().ok()
                                                                    };
                                                                    set_ports.set(current_ports);
                                                                }
                                                            />
                                                        </div>
                                                        <div>
                                                            <label style="display: block; margin-bottom: 5px; font-size: 12px;">"Container Port:"</label>
                                                            <input
                                                                type="number"
                                                                style="width: 100%; padding: 6px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                                prop:value=move || port.container_port.to_string()
                                                                on:input=move |ev| {
                                                                    let mut current_ports = ports.get();
                                                                    if let Ok(port_num) = event_target_value(&ev).parse() {
                                                                        current_ports[index].container_port = port_num;
                                                                        set_ports.set(current_ports);
                                                                    }
                                                                }
                                                            />
                                                        </div>
                                                        <div>
                                                            <label style="display: block; margin-bottom: 5px; font-size: 12px;">"Protocol:"</label>
                                                            <select
                                                                style="width: 100%; padding: 6px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                                on:change=move |ev| {
                                                                    let mut current_ports = ports.get();
                                                                    current_ports[index].protocol = event_target_value(&ev);
                                                                    set_ports.set(current_ports);
                                                                }
                                                            >
                                                                <option value="tcp" selected=port.protocol == "tcp">"TCP"</option>
                                                                <option value="udp" selected=port.protocol == "udp">"UDP"</option>
                                                            </select>
                                                        </div>
                                                        <button
                                                            style="padding: 6px 8px; background-color: #e74c3c; border: none; border-radius: 4px; color: white; cursor: pointer;"
                                                            on:click=move |_| {
                                                                let mut current_ports = ports.get();
                                                                current_ports.remove(index);
                                                                set_ports.set(current_ports);
                                                            }
                                                        >
                                                            "×"
                                                        </button>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>

                                    // Volume mounts
                                    <div style="margin-bottom: 30px;">
                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;">
                                            <h4 style="margin: 0;">"Volume Mounts:"</h4>
                                            <button class="btn-primary" style="padding: 5px 10px; font-size: 12px;" on:click=move |_| add_volume()>
                                                "Add Volume"
                                            </button>
                                        </div>
                                        <For
                                            each=move || volumes.get().into_iter().enumerate().collect::<Vec<_>>()
                                            key=|(i, _)| *i
                                            children=move |(index, volume)| {
                                                view! {
                                                    <div style="display: grid; grid-template-columns: 1fr 1fr auto auto; gap: 10px; margin-bottom: 10px; align-items: end;">
                                                        <div>
                                                            <label style="display: block; margin-bottom: 5px; font-size: 12px;">"Host Path:"</label>
                                                            <input
                                                                type="text"
                                                                style="width: 100%; padding: 6px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                                prop:value=move || volume.source.clone()
                                                                on:input=move |ev| {
                                                                    let mut current_volumes = volumes.get();
                                                                    current_volumes[index].source = event_target_value(&ev);
                                                                    set_volumes.set(current_volumes);
                                                                }
                                                            />
                                                        </div>
                                                        <div>
                                                            <label style="display: block; margin-bottom: 5px; font-size: 12px;">"Container Path:"</label>
                                                            <input
                                                                type="text"
                                                                style="width: 100%; padding: 6px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                                prop:value=move || volume.target.clone()
                                                                on:input=move |ev| {
                                                                    let mut current_volumes = volumes.get();
                                                                    current_volumes[index].target = event_target_value(&ev);
                                                                    set_volumes.set(current_volumes);
                                                                }
                                                            />
                                                        </div>
                                                        <label style="display: flex; align-items: center; gap: 5px; font-size: 12px; white-space: nowrap;">
                                                            <input
                                                                type="checkbox"
                                                                prop:checked=move || volume.read_only
                                                                on:change=move |ev| {
                                                                    let mut current_volumes = volumes.get();
                                                                    current_volumes[index].read_only = event_target_checked(&ev);
                                                                    set_volumes.set(current_volumes);
                                                                }
                                                            />
                                                            "Read Only"
                                                        </label>
                                                        <button
                                                            style="padding: 6px 8px; background-color: #e74c3c; border: none; border-radius: 4px; color: white; cursor: pointer;"
                                                            on:click=move |_| {
                                                                let mut current_volumes = volumes.get();
                                                                current_volumes.remove(index);
                                                                set_volumes.set(current_volumes);
                                                            }
                                                        >
                                                            "×"
                                                        </button>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>

                                    // Environment variables
                                    <div>
                                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;">
                                            <h4 style="margin: 0;">"Environment Variables:"</h4>
                                            <button class="btn-primary" style="padding: 5px 10px; font-size: 12px;" on:click=move |_| add_env_var()>
                                                "Add Variable"
                                            </button>
                                        </div>
                                        <For
                                            each=move || env_vars.get().into_iter().collect::<Vec<_>>()
                                            key=|(key, _)| key.clone()
                                            children=move |(key, value)| {
                                                let key_for_update = key.clone();
                                                let key_for_delete = key.clone();
                                                view! {
                                                    <div style="display: grid; grid-template-columns: 1fr 1fr auto; gap: 10px; margin-bottom: 10px; align-items: end;">
                                                        <div>
                                                            <label style="display: block; margin-bottom: 5px; font-size: 12px;">"Variable:"</label>
                                                            <input
                                                                type="text"
                                                                style="width: 100%; padding: 6px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                                prop:value=move || key.clone()
                                                                on:input=move |ev| {
                                                                    let mut current_env = env_vars.get();
                                                                    let new_key = event_target_value(&ev);
                                                                    if new_key != key_for_update {
                                                                        current_env.remove(&key_for_update);
                                                                        current_env.insert(new_key, value.clone());
                                                                        set_env_vars.set(current_env);
                                                                    }
                                                                }
                                                            />
                                                        </div>
                                                        <div>
                                                            <label style="display: block; margin-bottom: 5px; font-size: 12px;">"Value:"</label>
                                                            <input
                                                                type="text"
                                                                style="width: 100%; padding: 6px; border: 1px solid #4a5568; border-radius: 4px; background-color: #2c3e50; color: white;"
                                                                prop:value=move || value.clone()
                                                                on:input=move |ev| {
                                                                    let mut current_env = env_vars.get();
                                                                    current_env.insert(key.clone(), event_target_value(&ev));
                                                                    set_env_vars.set(current_env);
                                                                }
                                                            />
                                                        </div>
                                                        <button
                                                            style="padding: 6px 8px; background-color: #e74c3c; border: none; border-radius: 4px; color: white; cursor: pointer;"
                                                            on:click=move |_| {
                                                                let mut current_env = env_vars.get();
                                                                current_env.remove(&key_for_delete);
                                                                set_env_vars.set(current_env);
                                                            }
                                                        >
                                                            "×"
                                                        </button>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            }.into_view(),
                            4 => view! {
                                <div class="wizard-step">
                                    <h3>"Step 4: Review & Create"</h3>
                                    <p>"Review your container configuration before creation"</p>

                                    <div style="background-color: #34495e; padding: 20px; border-radius: 8px;">
                                        <h4 style="margin-top: 0; color: #3498db;">"Container Summary"</h4>

                                        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px;">
                                            <div>
                                                <div style="margin-bottom: 15px;">
                                                    <strong>"Name: "</strong>
                                                    <span style="color: #2ecc71;">{move || container_name.get()}</span>
                                                </div>
                                                <div style="margin-bottom: 15px;">
                                                    <strong>"Image: "</strong>
                                                    {move || {
                                                        if let Some(image) = selected_image.get() {
                                                            view! {
                                                                <span>
                                                                    <span style="color: #3498db;">{&image.name}</span>
                                                                    <span style="color: #f39c12;">":"</span>
                                                                    <span style="color: #2ecc71;">{&image.tag}</span>
                                                                </span>
                                                            }.into_view()
                                                        } else {
                                                            view! { <span style="color: #e74c3c;">"No image selected"</span> }.into_view()
                                                        }
                                                    }}
                                                </div>
                                                <div style="margin-bottom: 15px;">
                                                    <strong>"Restart Policy: "</strong>
                                                    <span style="color: #f39c12;">
                                                        {move || match restart_policy.get() {
                                                            RestartPolicy::No => "No",
                                                            RestartPolicy::Always => "Always",
                                                            RestartPolicy::UnlessStopped => "Unless Stopped",
                                                            RestartPolicy::OnFailure => "On Failure",
                                                        }}
                                                    </span>
                                                </div>
                                                <div style="margin-bottom: 15px;">
                                                    <strong>"Features: "</strong>
                                                    <div style="margin-top: 5px;">
                                                        {move || {
                                                            let mut features = Vec::new();
                                                            if enable_gaming.get() {
                                                                features.push("Gaming Mode");
                                                            }
                                                            if enable_gpu.get() {
                                                                features.push("GPU Access");
                                                            }
                                                            if features.is_empty() {
                                                                features.push("Standard");
                                                            }
                                                            features.join(", ")
                                                        }}
                                                    </div>
                                                </div>
                                            </div>

                                            <div>
                                                <div style="margin-bottom: 15px;">
                                                    <strong>"Ports: "</strong>
                                                    <div style="margin-top: 5px;">
                                                        {move || {
                                                            let port_list = ports.get();
                                                            if port_list.is_empty() {
                                                                "None".to_string()
                                                            } else {
                                                                port_list.iter().map(|p| {
                                                                    format!("{}:{}/{}",
                                                                        p.host_port.map(|hp| hp.to_string()).unwrap_or_else(|| "auto".to_string()),
                                                                        p.container_port,
                                                                        p.protocol
                                                                    )
                                                                }).collect::<Vec<_>>().join(", ")
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                                <div style="margin-bottom: 15px;">
                                                    <strong>"Volumes: "</strong>
                                                    <div style="margin-top: 5px;">
                                                        {move || {
                                                            let volume_list = volumes.get();
                                                            if volume_list.is_empty() {
                                                                "None".to_string()
                                                            } else {
                                                                format!("{} volume(s)", volume_list.len())
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                                <div style="margin-bottom: 15px;">
                                                    <strong>"Environment: "</strong>
                                                    <div style="margin-top: 5px;">
                                                        {move || {
                                                            let env_count = env_vars.get().len();
                                                            if env_count == 0 {
                                                                "No variables".to_string()
                                                            } else {
                                                                format!("{} variable(s)", env_count)
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            }.into_view(),
                            _ => view! { <div>"Invalid step"</div> }.into_view()
                        }
                    }}
                </div>

                // Navigation buttons
                <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 20px; border-top: 1px solid #4a5568; padding-top: 15px;">
                    <button
                        class="btn-primary"
                        style="background-color: #6c757d;"
                        on:click=move |_| {
                            if current_step.get() > 1 {
                                set_current_step.set(current_step.get() - 1);
                            }
                        }
                        disabled=move || current_step.get() == 1
                    >
                        "Previous"
                    </button>

                    <div style="display: flex; gap: 10px;">
                        {move || {
                            let step = current_step.get();
                            if step < 4 {
                                view! {
                                    <button
                                        class="btn-primary"
                                        on:click=move |_| {
                                            set_current_step.set(current_step.get() + 1);
                                        }
                                        disabled=move || {
                                            let step = current_step.get();
                                            match step {
                                                1 => selected_image.get().is_none(),
                                                2 => container_name.get().is_empty(),
                                                _ => false
                                            }
                                        }
                                    >
                                        "Next"
                                    </button>
                                }.into_view()
                            } else {
                                view! {
                                    <button
                                        class="btn-success"
                                        on:click=move |_| create_container()
                                        disabled=move || loading.get() || selected_image.get().is_none() || container_name.get().is_empty()
                                    >
                                        {if loading.get() { "Creating..." } else { "Create Container" }}
                                    </button>
                                }.into_view()
                            }
                        }}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Load registries for the wizard
async fn load_registries_for_wizard(
    set_registries: WriteSignal<Vec<RegistryConfig>>,
) {
    match Request::get("http://localhost:8000/api/v1/registries")
        .send()
        .await
    {
        Ok(response) => {
            if let Ok(registry_list) = response.json::<Vec<RegistryConfig>>().await {
                set_registries.set(registry_list);
            }
        }
        Err(_) => {
            // Silently handle error, user can still manually enter image names
        }
    }
}