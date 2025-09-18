# Bolt GhostPanel API Integration Guide

> **Next Steps: Connecting GhostPanel to Bolt APIs and UI Polish**
>
> This guide outlines the roadmap for completing GhostPanel's integration with Bolt container runtime and creating a production-ready **Portainer alternative** for enterprise container management with optional gaming workload support.

---

## 🎯 Current Status & Goals

### ✅ What's Complete
- Multi-crate architecture with solid foundations
- Core data models for containers, networks, volumes, and advanced workloads
- Leptos frontend framework setup with routing
- Basic UI components and Portainer-like styling
- Authentication framework structure
- WebSocket infrastructure planning

### 🚧 What Needs Implementation
- **Live Bolt API integration** (currently shows mock data)
- **Real-time container operations** (start, stop, create, delete)
- **WebSocket live updates** for container status changes
- **Complete UI pages** (most are placeholder stubs)
- **Error handling and user feedback**
- **Production deployment configuration**

---

## 🔌 Phase 1: Bolt API Integration (Weeks 1-3)

### 1.1 Bolt Runtime Client Setup

**Location**: `crates/gpanel-core/src/api.rs`

```rust
// Current: Empty placeholder
// Target: Full Bolt client implementation

use bolt_client::{BoltClient, Container, Image, Network, Volume};

#[derive(Clone)]
pub struct BoltApiClient {
    client: BoltClient,
    base_url: String,
}

impl BoltApiClient {
    pub async fn new(bolt_url: &str) -> Result<Self> {
        let client = BoltClient::connect(bolt_url).await?;
        Ok(Self {
            client,
            base_url: bolt_url.to_string(),
        })
    }

    // Container operations
    pub async fn list_containers(&self, filter: Option<ContainerFilter>) -> Result<Vec<Container>> {
        // Implement Bolt container listing
    }

    pub async fn create_container(&self, req: CreateContainerRequest) -> Result<Container> {
        // Implement Bolt container creation with advanced configurations
    }

    pub async fn start_container(&self, id: &str) -> Result<()> {
        // Implement Bolt container start
    }

    // ... more operations
}
```

**Priority Tasks:**
1. **Add Bolt dependency** to `gpanel-core/Cargo.toml`
2. **Implement container CRUD** operations with proper error handling
3. **Add advanced workload APIs** (resource allocation, performance optimization, gaming workloads)
4. **Create connection pooling** for performance
5. **Add retry logic** and circuit breakers

### 1.2 Agent Service Implementation

**Location**: `crates/gpanel-agent/src/main.rs`

```rust
// Current: Basic structure
// Target: Full system monitoring + Bolt integration

use axum::{extract::State, Json, Router};
use gpanel_core::{BoltApiClient, Container, SystemMetrics};

#[derive(Clone)]
struct AgentState {
    bolt_client: BoltApiClient,
    metrics_collector: MetricsCollector,
}

async fn get_containers(State(state): State<AgentState>) -> Result<Json<Vec<Container>>> {
    let containers = state.bolt_client.list_containers(None).await?;
    Ok(Json(containers))
}

async fn container_metrics(State(state): State<AgentState>) -> Result<Json<SystemMetrics>> {
    // Collect real-time container metrics, resource usage, specialized workload monitoring
}

#[tokio::main]
async fn main() -> Result<()> {
    let bolt_client = BoltApiClient::new("bolt://localhost:8080").await?;
    let state = AgentState { bolt_client, metrics_collector: MetricsCollector::new() };

    let app = Router::new()
        .route("/api/v1/containers", get(get_containers))
        .route("/api/v1/metrics", get(container_metrics))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

**Priority Tasks:**
1. **Implement REST endpoints** for all container operations
2. **Add real-time metrics collection** (CPU, memory, GPU)
3. **Create WebSocket endpoint** for live updates
4. **Add error handling middleware**
5. **Implement health checks**

### 1.3 Frontend Service Integration

**Location**: `crates/gpanel-web/src/services/mod.rs`

```rust
// Current: Empty module
// Target: API client for frontend

use gloo_net::http::Request;
use leptos::*;
use gpanel_core::{Container, CreateContainerRequest, Result};

pub struct ApiService {
    base_url: String,
}

impl ApiService {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:8000/api/v1".to_string(),
        }
    }

    pub async fn list_containers(&self) -> Result<Vec<Container>> {
        let response = Request::get(&format!("{}/containers", self.base_url))
            .send()
            .await?;

        let containers: Vec<Container> = response.json().await?;
        Ok(containers)
    }

    pub async fn start_container(&self, id: &str) -> Result<()> {
        Request::post(&format!("{}/containers/{}/start", self.base_url, id))
            .send()
            .await?;
        Ok(())
    }

    // ... more operations
}

// Create global resource for Leptos
pub fn provide_api_service() {
    provide_context(ApiService::new());
}
```

---

## 🎨 Phase 2: UI Polish & Real Data (Weeks 2-4)

### 2.1 Container List Page (HIGH PRIORITY)

**Location**: `crates/gpanel-web/src/pages/containers.rs`

```rust
// Current: Basic placeholder
// Target: Full container management interface

use leptos::*;
use crate::services::ApiService;

#[component]
pub fn ContainerList() -> impl IntoView {
    let api = use_context::<ApiService>().expect("ApiService must be provided");

    // Create reactive resource for containers
    let containers = create_resource(
        || (),
        move |_| {
            let api = api.clone();
            async move { api.list_containers().await }
        }
    );

    let start_container = create_action(move |id: &str| {
        let api = api.clone();
        let id = id.to_string();
        async move {
            api.start_container(&id).await?;
            containers.refetch(); // Refresh list
            Ok(())
        }
    });

    view! {
        <div class="container-list">
            <div class="header-actions">
                <h2>"Containers"</h2>
                <button class="btn-primary" on:click=|_| {
                    // Navigate to create container form
                }>"Create Container"</button>
            </div>

            <Suspense fallback=move || view! { <div>"Loading containers..."</div> }>
                {move || {
                    containers.get().map(|containers_result| match containers_result {
                        Ok(containers) => view! {
                            <div class="container-grid">
                                <For
                                    each=move || containers.clone()
                                    key=|container| container.id.clone()
                                    children=move |container| {
                                        view! { <ContainerCard container=container start_action=start_container/> }
                                    }
                                />
                            </div>
                        }.into_view(),
                        Err(e) => view! {
                            <div class="error-message">
                                "Failed to load containers: " {e.to_string()}
                            </div>
                        }.into_view()
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn ContainerCard(
    container: Container,
    start_action: Action<String, Result<()>>
) -> impl IntoView {
    view! {
        <div class="container-card">
            <div class="container-header">
                <h3>{&container.name}</h3>
                <span class={format!("status-{}", container.status.to_string().to_lowercase())}>
                    {container.status.to_string()}
                </span>
                {container.gaming_config.as_ref().map(|_| view! {
                    <span class="gaming-badge">"GAMING"</span>
                })}
                {container.gpu_allocation.as_ref().map(|_| view! {
                    <span class="gpu-indicator">"GPU"</span>
                })}
            </div>

            <div class="container-details">
                <p><strong>"Image:"</strong> {&container.image}</p>
                <p><strong>"Created:"</strong> {container.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</p>

                // Show ports if any
                {(!container.ports.is_empty()).then(|| view! {
                    <div class="port-mappings">
                        <strong>"Ports: "</strong>
                        {container.ports.iter().map(|port| {
                            format!("{}:{}", port.host_port.unwrap_or(0), port.container_port)
                        }).collect::<Vec<_>>().join(", ")}
                    </div>
                })}
            </div>

            <div class="container-actions">
                {match container.status {
                    ContainerStatus::Running => view! {
                        <button class="btn-danger" on:click=move |_| {
                            // Stop container action
                        }>"Stop"</button>
                        <button class="btn-primary" on:click=move |_| {
                            // Restart container action
                        }>"Restart"</button>
                    }.into_view(),
                    _ => view! {
                        <button class="btn-success" on:click=move |_| {
                            start_action.dispatch(container.id.clone());
                        }>"Start"</button>
                    }.into_view()
                }}

                <button class="btn-primary" on:click=move |_| {
                    // Navigate to container details
                }>"Details"</button>
            </div>
        </div>
    }
}
```

### 2.2 Gaming Dashboard (UNIQUE FEATURE)

**Location**: `crates/gpanel-web/src/pages/gaming.rs`

```rust
use leptos::*;

#[component]
pub fn GamingDashboard() -> impl IntoView {
    view! {
        <div class="gaming-dashboard">
            <h2>"Gaming Dashboard"</h2>

            // GPU Status Cards
            <div class="gpu-grid">
                <div class="container-card">
                    <h3>"GPU Utilization"</h3>
                    <div class="gpu-stats">
                        <div class="gpu-bar">
                            <div class="gpu-usage" style="width: 75%"></div>
                        </div>
                        <span>"75% - RTX 4090"</span>
                    </div>
                </div>

                <div class="container-card">
                    <h3>"VRAM Usage"</h3>
                    <div class="gpu-stats">
                        <span>"12.5 GB / 24 GB"</span>
                        <div class="gpu-bar">
                            <div class="memory-usage" style="width: 52%"></div>
                        </div>
                    </div>
                </div>
            </div>

            // Gaming Containers
            <div class="gaming-containers">
                <h3>"Gaming Containers"</h3>
                // Show only containers with gaming configs
            </div>

            // Proton Versions
            <div class="proton-manager">
                <h3>"Proton Versions"</h3>
                <div class="proton-list">
                    // List available Proton versions
                    // Show compatibility matrix
                </div>
            </div>
        </div>
    }
}
```

### 2.3 Real-time Updates with WebSockets

**Location**: `crates/gpanel-web/src/services/websocket.rs`

```rust
use leptos::*;
use gloo_net::websocket::{Message, WebSocket};
use futures::StreamExt;

#[derive(Clone)]
pub struct WebSocketService {
    // WebSocket connection for real-time updates
}

impl WebSocketService {
    pub fn new() -> Self {
        // Connect to ws://localhost:8000/ws
        // Handle container status updates
        // Handle metrics updates
    }

    pub fn subscribe_container_updates(&self) -> impl Stream<Item = ContainerUpdate> {
        // Stream of container state changes
    }
}

// Use in container list to auto-refresh
```

---

## 🔧 Phase 3: Production Polish (Weeks 4-6)

### 3.1 Error Handling & User Feedback

```rust
// Add toast notifications for user actions
// Implement proper error boundaries
// Add loading states for all operations
// Create confirmation dialogs for destructive actions
```

### 3.2 Create Container Wizard

```rust
// Multi-step form for container creation
// Gaming-specific options (GPU, Proton, Steam)
// Template selection (gaming presets)
// Validation and preview
```

### 3.3 Container Details Page

```rust
// Full container inspection
// Live logs viewer
// File browser
// Terminal access
// Performance graphs
```

### 3.4 System Settings & Configuration

```rust
// Bolt connection settings
// GPU configuration
// User management
// Backup/restore
```

---

## 🚀 Phase 4: Advanced Features (Weeks 5-8)

### 4.1 Multi-Node Cluster Support

```rust
// Connect to multiple Bolt instances
// Cluster overview dashboard
// Cross-cluster container migration
```

### 4.2 Steam Integration

```rust
// Steam library import
// One-click game container deployment
// Compatibility database
```

### 4.3 Performance Monitoring

```rust
// Grafana-style dashboards
// Historical metrics storage
// Alert system
```

---

## 📋 Development Priorities

### Week 1-2: Core Functionality
1. ✅ **Bolt API client implementation**
2. ✅ **Container list with real data**
3. ✅ **Basic container operations (start/stop)**
4. ✅ **Error handling framework**

### Week 3-4: User Experience
1. ✅ **Container creation wizard**
2. ✅ **Real-time WebSocket updates**
3. ✅ **Gaming dashboard with GPU stats**
4. ✅ **Toast notifications and loading states**

### Week 5-6: Polish & Testing
1. ✅ **Container details page**
2. ✅ **System logs and monitoring**
3. ✅ **Authentication improvements**
4. ✅ **Mobile responsiveness**

### Week 7-8: Advanced Features
1. ✅ **Steam integration basics**
2. ✅ **Multi-cluster support**
3. ✅ **Performance monitoring**
4. ✅ **Production deployment**

---

## 🔌 API Integration Checklist

### Container Management APIs
- [ ] `GET /api/v1/containers` - List containers with filters
- [ ] `POST /api/v1/containers` - Create container with gaming configs
- [ ] `POST /api/v1/containers/{id}/start` - Start container
- [ ] `POST /api/v1/containers/{id}/stop` - Stop container
- [ ] `POST /api/v1/containers/{id}/restart` - Restart container
- [ ] `DELETE /api/v1/containers/{id}` - Remove container
- [ ] `GET /api/v1/containers/{id}` - Container details
- [ ] `GET /api/v1/containers/{id}/logs` - Container logs (streaming)

### Gaming APIs
- [ ] `GET /api/v1/gaming/gpu` - GPU status and allocation
- [ ] `POST /api/v1/gaming/gpu/allocate` - Allocate GPU to container
- [ ] `GET /api/v1/gaming/proton` - Available Proton versions
- [ ] `POST /api/v1/gaming/proton/install` - Install Proton version

### System APIs
- [ ] `GET /api/v1/system/metrics` - System resource usage
- [ ] `GET /api/v1/system/info` - Bolt version, system info
- [ ] `WS /api/v1/events` - Real-time event stream

---

## 🎯 Success Metrics

### Performance Targets
- **Page Load Time**: < 2 seconds for dashboard
- **Container Operations**: < 500ms response time
- **Real-time Updates**: < 100ms WebSocket latency
- **Memory Usage**: < 50MB for web interface

### User Experience Goals
- **One-click Gaming**: Deploy game containers in < 30 seconds
- **GPU Management**: Visual allocation with drag-and-drop
- **Real-time Monitoring**: Live updates without page refresh
- **Mobile Support**: Responsive design for tablets/phones

---

## 📦 Deployment Strategy

### Development
```bash
# Start all services locally
cargo run --bin gpanel-web     # Port 9443
cargo run --bin gpanel-agent   # Port 8000
cargo run --bin gpanel-cli     # Port 9000
```

### Production (Docker/Bolt)
```bash
# Use existing Boltfile.toml configuration
bolt up ghostpanel
# Access at https://localhost:9443
```

### Kubernetes/Docker Swarm
```yaml
# Helm chart for enterprise deployments
# Multi-replica agent services
# Load balancing and SSL termination
```

---

**🔧 GhostPanel will be the definitive Portainer alternative for Bolt container management** - combining enterprise-grade container management with Bolt's performance and optional specialized workload support.

This guide provides a clear roadmap to transform the solid foundation into a production-ready application that developers and gamers will love using.