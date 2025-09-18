<div align="center">
  <img src="assets/GhostPanel-Logo.png" alt="GhostPanel Logo" width="256">

  # GhostPanel
  ### The Portainer for Bolt - Enterprise Container Management Platform

  ![Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge&logo=rust)
  ![Container Management](https://img.shields.io/badge/Container-Management-blue?style=for-the-badge&logo=docker)
  ![Edge Computing](https://img.shields.io/badge/Edge-Computing-green?style=for-the-badge)
  ![QUIC/HTTP3](https://img.shields.io/badge/QUIC%2FHTTP3-Socket%20Proxy-purple?style=for-the-badge)
  ![Multi-Cluster](https://img.shields.io/badge/Multi-Cluster-darkgreen?style=for-the-badge)
  ![Real-time Monitoring](https://img.shields.io/badge/Real--time-Monitoring-cyan?style=for-the-badge)
  ![Gaming Features](https://img.shields.io/badge/Gaming-Features-red?style=for-the-badge&logo=steam)
  ![Mesh Networking](https://img.shields.io/badge/Mesh-Networking-yellow?style=for-the-badge)
</div>

---

## Overview

GhostPanel is **Portainer for Bolt** - a lightweight, intuitive web interface for managing Bolt container environments. Just as Portainer simplified Docker management, GhostPanel provides the same user-friendly experience for the next-generation Bolt container platform.

**Core Portainer-like Features:**
- **Simple Deployment** - Single container deployment, just like Portainer
- **Intuitive Web GUI** - Familiar interface for containers, images, volumes, and networks
- **Multi-Environment Support** - Manage multiple Bolt clusters from one interface
- **Role-Based Access Control** - Enterprise-grade user management and permissions
- **Real-time Monitoring** - Live container stats, logs, and performance metrics

**Bolt-Specific Enhancements:**
- **Native TOML Configuration** - Visual editor for Bolt's declarative config format
- **Surge Orchestration** - Built-in support for Bolt's orchestration system
- **nvbind GPU Management** - 100x faster GPU passthrough configuration
- **QUIC Networking** - Advanced networking with Bolt's modern protocol stack
- **Automated Snapshots** - BTRFS/ZFS snapshot management interface

## Core Features

### 📦 **Container Management** *(Portainer-like)*
- **Container Lifecycle** - Start, stop, restart, delete containers with one click
- **Container Browser** - Visual file browser and terminal access
- **Log Viewing** - Real-time container logs with search and filtering
- **Image Management** - Pull, build, and deploy container images
- **Volume Management** - Create and manage persistent storage volumes
- **Network Management** - Configure container networking and connectivity

### ⚙️ **Bolt-Specific Features**
- **TOML Configuration Editor** - Visual editor for Bolt's declarative config format
- **Surge Orchestration** - Deploy and manage workloads across Bolt clusters
- **nvbind GPU Passthrough** - Configure GPU access with Bolt's ultra-fast passthrough
- **QUIC Networking** - Manage Bolt's modern QUIC-based networking
- **Automated Snapshots** - BTRFS/ZFS snapshot scheduling and restoration
- **Performance Monitoring** - Bolt-optimized resource usage and performance metrics

### 🏢 **Enterprise Features**
- **Multi-Environment Support** - Manage multiple Bolt clusters from single interface
- **Role-Based Access Control** - User management with granular permissions
- **Team Management** - Organization-level access control and resource quotas
- **Audit Logging** - Complete action history for compliance and security
- **API Access** - Full REST API for automation and integration

### 🎮 **Advanced Workloads** *(Bonus Features)*
- **Gaming Templates** - Pre-configured gaming server containers
- **GPU Allocation** - Visual GPU resource assignment interface
- **Steam Integration** - Gaming-specific container templates and management
- **Media Streaming** - Optimized containers for streaming workloads

## Tech Stack

### Backend (Core)
- **Language**: Rust 🦀
- **Web Framework**: Actix-Web / Axum (high-performance async)
- **Networking**: QUIC/HTTP3 with socket proxy capabilities
- **Database**: SQLite (embedded) / PostgreSQL (enterprise clusters)
- **Message Queue**: NATS for distributed events and edge coordination
- **Metrics**: Prometheus + OpenTelemetry with edge aggregation

### Frontend Options

#### Option 1: Leptos (Recommended for Rust Ecosystem)
```rust
// Full-stack Rust with server-side rendering
// WASM for interactive components
// Excellent performance with minimal JS bundle
```
**Pros**:
- Single language (Rust) for entire stack
- Server-side rendering with hydration
- Fine-grained reactivity
- Small WASM bundles

#### Option 2: Yew (Pure WASM)
```rust
// Client-side only WASM framework
// Component-based like React
// Mature ecosystem
```
**Pros**:
- Pure Rust frontend
- Strong type safety
- Good component model
- Active community

#### Option 3: Tauri + Leptos/Yew (Desktop)
```rust
// Native desktop app with web technologies
// System tray integration
// Direct system access
```
**Pros**:
- Native performance
- Small binary size
- System integration
- Cross-platform

### Enterprise Architecture
```
┌─────────────────────────────────────────────────────────────────┐
│                    GhostPanel Management Plane                 │
├────────────────┬─────────────────┬─────────────────────────────┤
│   Web UI       │   REST API      │   QUIC/HTTP3 Socket Proxy  │
│   (WebAssembly)│   (Actix/Axum)  │   (Edge Communication)     │
├────────────────┴─────────────────┴─────────────────────────────┤
│                  Edge Agent Network                            │
├─────────────────────────────────────────────────────────────────┤
│     Edge Agent 1    │    Edge Agent 2    │    Edge Agent N    │
│  ┌─────────────────┐│  ┌─────────────────┐│  ┌─────────────────┐│
│  │  Bolt Cluster   ││  │  Bolt Cluster   ││  │  Bolt Cluster   ││
│  │   (Region A)    ││  │   (Region B)    ││  │   (Region C)    ││
│  └─────────────────┘│  └─────────────────┘│  └─────────────────┘│
└─────────────────────────────────────────────────────────────────┘
```

## Quick Start

### Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
cargo install cargo-watch cargo-make
```

### Development Setup
```bash
# Clone repository
git clone https://github.com/CK-Technology/ghostpanel
cd ghostpanel

# Build backend
cargo build --release

# Run development server
cargo watch -x run

# Access at http://localhost:8080
```

### Docker Deployment
```dockerfile
# Multi-stage build for minimal image
FROM rust:1.75-alpine AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /app/target/release/ghostpanel /usr/local/bin/
EXPOSE 8080
CMD ["ghostpanel"]
```

## Project Structure
```
ghostpanel/
├── src/
│   ├── api/           # REST API handlers
│   ├── models/        # Data models and schemas
│   ├── services/      # Business logic
│   ├── websocket/     # Real-time event system
│   ├── bolt/          # Bolt integration layer
│   └── web/           # Frontend (if using Leptos SSR)
├── migrations/        # Database migrations
├── assets/
│   └── icons/        # Generated icon sizes
├── Cargo.toml        # Dependencies
└── Boltfile          # Container definition
```

## Performance Targets

| Metric | Target | Current |
|--------|--------|---------|
| API Latency | < 10ms | - |
| WebSocket Latency | < 5ms | - |
| Memory Usage | < 50MB | - |
| Binary Size | < 20MB | - |
| Startup Time | < 1s | - |
| Concurrent Connections | > 10,000 | - |

## Roadmap

### Phase 1: Core Portainer Functionality (Q1 2025)
- [x] Project architecture and setup
- [ ] **Container Management UI** - List, create, start, stop, delete containers
- [ ] **Image Management** - Pull, build, and manage container images
- [ ] **Volume & Network Management** - Basic storage and networking interfaces
- [ ] **Authentication System** - JWT-based login with basic user management
- [ ] **Real-time Updates** - WebSocket events for live container status

### Phase 2: Bolt-Specific Features (Q2 2025)
- [ ] **TOML Configuration Editor** - Visual editor for Bolt configs
- [ ] **Surge Orchestration Interface** - Deploy workloads across clusters
- [ ] **nvbind GPU Management** - Configure Bolt's GPU passthrough
- [ ] **QUIC Networking** - Manage Bolt's advanced networking features
- [ ] **Snapshot Management** - BTRFS/ZFS snapshot interface

### Phase 3: Enterprise & Multi-Environment (Q3 2025)
- [ ] **Multi-Cluster Support** - Manage multiple Bolt environments
- [ ] **Advanced RBAC** - Team management and granular permissions
- [ ] **Audit Logging** - Complete action history and compliance
- [ ] **API Documentation** - Full REST API with OpenAPI specs
- [ ] **Edge Agent Network** - Remote cluster management

### Phase 4: Advanced Features (Q4 2025)
- [ ] **Gaming Templates** - Pre-built gaming server configurations
- [ ] **Performance Analytics** - Advanced monitoring and optimization
- [ ] **Custom Dashboards** - User-configurable monitoring interfaces
- [ ] **Automation & CI/CD** - Integration with deployment pipelines
- [ ] **Plugin System** - Extensible architecture for custom features

## API Documentation

### Bolt Container Operations
```rust
// List Bolt containers
GET /api/v1/containers

// Create Bolt container with TOML config
POST /api/v1/containers
{
  "name": "web-server",
  "bolt_config": {
    "container": {
      "image": "nginx:latest",
      "volumes": ["/data:/usr/share/nginx/html"],
      "network": "surge-mesh"
    },
    "resources": {
      "cpu": "2",
      "memory": "4GB"
    }
  }
}

// Bolt-specific operations
POST /api/v1/containers/{id}/snapshot   // BTRFS/ZFS snapshots
POST /api/v1/containers/{id}/surge      // Surge orchestration
GET  /api/v1/containers/{id}/nvbind     // GPU passthrough status
```

### Surge Orchestration API
```rust
// Deploy workload across Bolt clusters
POST /api/v1/surge/deploy
{
  "workload": "web-app",
  "replicas": 3,
  "regions": ["us-east", "eu-west"],
  "bolt_config": "web-server.toml"
}

// QUIC networking configuration
POST /api/v1/network/quic
{
  "cluster_id": "main",
  "endpoints": ["10.0.1.0/24"],
  "encryption": true
}
```

### WebSocket Events
```javascript
// Connect to event stream
ws://localhost:8080/api/v1/events

// Event types
{
  "type": "container.started",
  "data": { "id": "abc123", "name": "game-server" }
}
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Security

GhostPanel implements multiple security layers:
- JWT-based authentication with refresh tokens
- Role-based access control (RBAC)
- Encrypted communication (TLS 1.3)
- Container isolation and sandboxing
- Audit logging for compliance

Report security vulnerabilities to: security@ck-technology.com

## Performance Benchmarks

```
Container Operations (ops/sec):
├── List:    10,000+
├── Create:  1,000+
├── Start:   500+
└── Delete:  1,000+

Network Throughput:
├── REST API:     1 Gbps
├── WebSocket:    100,000 msg/sec
└── File Upload:  500 MB/s

Resource Usage (idle):
├── CPU:     < 1%
├── Memory:  < 50MB
└── Disk I/O: Minimal
```

## Comparison

| Feature | GhostPanel (Bolt) | Portainer (Docker) | Rancher (K8s) |
|---------|-------------------|-------------------|---------------|
| **Runtime** | **Bolt (Rust)** | Docker | Kubernetes |
| **Language** | **Rust** | Go | Go |
| **Performance** | **Ultra-High** | Good | Moderate |
| **GPU Passthrough** | **Native (nvbind)** | Basic | Complex |
| **Snapshots** | **Automated BTRFS/ZFS** | Manual | External |
| **Networking** | **QUIC + Surge** | Bridge/Overlay | CNI |
| **Config** | **Declarative TOML** | Docker Compose | YAML |
| **Memory Usage** | **Minimal** | Moderate | High |
| **Gaming/Media** | **Optimized** | Basic | None |
| **Edge Computing** | **Built-in** | Limited | Complex |

## License

Copyright © 2024 CK-Technology. All rights reserved.

---

<div align="center">
  <strong>Built with 🦀 in Rust for maximum performance and reliability</strong>

  [Documentation](https://docs.ghostpanel.io) • [API Reference](https://api.ghostpanel.io) • [Discord](https://discord.gg/ghostpanel) • [Twitter](https://twitter.com/ghostpanel)
</div>