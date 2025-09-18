# GhostPanel Web Admin Plan

> **The Ultimate Portainer Alternative for Bolt Containers**
> A modern, gaming-focused container management interface built with Rust + Leptos

---

## 🎯 Project Vision

GhostPanel is a next-generation web interface that brings Portainer-like functionality to the Bolt container ecosystem. Think Portainer meets gaming optimization with Rust performance and modern web technologies.

**Key Differentiators:**
- 🚀 **Built for Bolt**: First-class integration with Bolt's gaming-optimized containers
- ⚡ **Rust + WASM**: Leptos frontend for near-native performance
- 🎮 **Gaming-First**: GPU monitoring, Proton management, Steam integration
- 🌐 **Modern Stack**: WebSockets, real-time updates, responsive design
- 🔧 **Developer-Friendly**: Hot reload, TypeScript-like safety with Rust

---

## 🏗️ Architecture Overview

### Multi-Service Architecture
```
┌─────────────────────────────────────────────────────────────┐
│                     GhostPanel Suite                        │
├─────────────────────────────────────────────────────────────┤
│  Web UI (Leptos)     │  Agent Service   │  CLI Bridge      │
│  Port 9443 (HTTPS)   │  Port 8000       │  Port 9000       │
│  - Container mgmt    │  - System stats   │  - Commands      │
│  - Real-time UI      │  - Health checks  │  - Automation    │
│  - Gaming dashboards │  - GPU monitoring │  - Scripting     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │   Bolt Runtime  │
                    │   (Your API)    │
                    │  - Containers   │
                    │  - Gaming       │
                    │  - GPU Support  │
                    └─────────────────┘
```

### Technology Stack

**Frontend (gpanel-web - Port 9443)**
- **Framework**: Leptos 0.6 (Rust + WASM)
- **Styling**: TailwindCSS + CSS Modules
- **State**: Leptos reactive signals
- **Routing**: leptos_router
- **HTTP**: gloo-net for API calls
- **Real-time**: WebSocket integration

**Agent Service (gpanel-agent - Port 8000)**
- **Framework**: Axum web server
- **Purpose**: System monitoring, health checks, metrics
- **Features**: GPU stats, container stats, system resources
- **Protocol**: REST API + WebSocket streams

**CLI Bridge (gpanel-cli - Port 9000)**
- **Framework**: Axum + Clap
- **Purpose**: Command-line interface proxy
- **Features**: Automation, scripting, bulk operations
- **Protocol**: REST API

**Core Library (gpanel-core)**
- **Purpose**: Shared types, utilities, Bolt integration
- **Features**: Error handling, config, container models

---

## 🎨 UI/UX Design Philosophy

### Portainer-Inspired Interface
- **Dark Theme Primary**: Gaming-focused, reduces eye strain
- **Familiar Layout**: Left sidebar navigation like Portainer
- **Card-Based Design**: Container cards, service cards, stats cards
- **Responsive**: Works on phones, tablets, desktops, ultrawide

### Modern Enhancements
- **Real-time Updates**: Live container stats without page refresh
- **Gaming Features**: GPU utilization, Proton compatibility, Steam integration
- **Performance Focus**: Sub-100ms UI updates via WASM
- **Drag & Drop**: Container deployment, file uploads

---

## 📋 Feature Roadmap

### Phase 1: Core Foundation (MVP) ✅ **STARTING NOW**
- [x] Project architecture and workspace setup
- [ ] **Basic Container Management**: List, start, stop, restart containers
- [ ] **Authentication**: JWT-based login system
- [ ] **Real-time Updates**: WebSocket container status updates
- [ ] **Responsive Design**: Mobile-friendly Portainer-like interface
- [ ] **Dark Theme**: Gaming-focused UI design

### Phase 2: Gaming Integration
- [ ] **GPU Dashboard**: Real-time GPU utilization and memory
- [ ] **Proton Manager**: Version selection and compatibility matrix
- [ ] **Steam Integration**: Game container templates and launching
- [ ] **Performance Monitoring**: FPS, latency, resource usage
- [ ] **Gaming Profiles**: Pre-configured gaming container setups

### Phase 3: Advanced Features
- [ ] **Multi-Node Management**: Distributed Bolt cluster support
- [ ] **Network Visualization**: Container networking topology
- [ ] **Volume Management**: Storage and backup interfaces
- [ ] **Log Aggregation**: Centralized container log viewing
- [ ] **Alert System**: Custom notifications and monitoring

### Phase 4: Enterprise & Polish
- [ ] **RBAC**: Role-based access control
- [ ] **API Documentation**: OpenAPI/Swagger integration
- [ ] **Metrics Dashboards**: Grafana-style performance charts
- [ ] **Backup/Restore**: Container state management
- [ ] **Plugins**: Extensible architecture

---

## 🔧 Port Configuration

| Port | Service | Protocol | Purpose |
|------|---------|----------|---------|
| **9443** | Web UI | HTTPS | Main web interface (Leptos frontend) |
| **8000** | Agent | HTTP/WS | System monitoring & container stats |
| **9000** | CLI Bridge | HTTP | Command automation & scripting |

### Security Considerations
- **HTTPS Only**: Web UI uses TLS encryption
- **JWT Auth**: Token-based authentication across services
- **CORS**: Properly configured cross-origin requests
- **Input Validation**: Rust's type system prevents injection attacks

---

## 🚀 Development Workflow

### Getting Started
```bash
# Clone and setup
git clone https://github.com/CK-Technology/ghostpanel
cd ghostpanel

# Install dependencies
cargo build

# Start development servers
cargo run --bin gpanel-web     # Port 9443
cargo run --bin gpanel-agent   # Port 8000
cargo run --bin gpanel-cli     # Port 9000
```

### Development Tools
- **Hot Reload**: Leptos supports hot reloading during development
- **Type Safety**: Full Rust type checking for frontend and backend
- **Testing**: Unit tests with cargo test, integration tests
- **Linting**: cargo clippy for code quality

---

## 🎮 Gaming-Specific Features

### GPU Management Interface
- **Real-time Monitoring**: GPU utilization, temperature, memory usage
- **Driver Management**: NVIDIA/AMD driver status and updates
- **Container Assignment**: Visual GPU allocation to containers
- **Performance Profiles**: Gaming vs compute optimizations

### Steam Integration Dashboard
- **Game Library**: Import Steam games as container templates
- **Proton Compatibility**: Automatic compatibility checking and recommendations
- **One-Click Launch**: Deploy and launch games with single click
- **Performance Tracking**: FPS monitoring, benchmark comparisons

### Container Optimizations
- **Gaming Templates**: Pre-configured containers for popular games
- **Resource Allocation**: Priority CPU/GPU assignment for gaming containers
- **Network Optimization**: Low-latency networking for online games
- **Storage Management**: Fast SSD allocation for game data

---

## 🔗 Bolt Integration Strategy

### API Integration
- **Bolt Runtime Client**: Direct integration with Bolt's Rust API
- **Container Lifecycle**: Full CRUD operations via Bolt
- **Gaming Features**: GPU passthrough, Proton management
- **Network Management**: QUIC networking, mesh topology

### Data Flow
```
GhostPanel Web UI (Leptos)
    ↓ HTTP/WebSocket
GhostPanel Agent (Axum)
    ↓ Rust API calls
Bolt Runtime
    ↓ System calls
Containers + GPU
```

---

## 📊 Success Metrics

### Performance Targets
- **Web UI Load Time**: < 2 seconds first load
- **Real-time Updates**: < 50ms WebSocket latency
- **Container Operations**: < 500ms response time
- **Memory Usage**: < 50MB RAM for web interface

### Gaming-Specific Goals
- **GPU Monitoring**: Real-time updates with < 100ms delay
- **Container Deployment**: < 30 seconds from click to running
- **Steam Integration**: One-click import of entire library
- **Performance Overhead**: < 5% impact on gaming performance

---

## 🛠️ Implementation Status

### ✅ Completed
- [x] Project architecture design
- [x] Workspace setup with 4 crates
- [x] Technology stack selection (Leptos + Axum)
- [x] Port configuration (9443, 8000, 9000)
- [x] Core types and configuration structure

### 🚧 In Progress
- [ ] Core error handling and utilities
- [ ] Basic Leptos web interface structure
- [ ] Agent service REST API foundation
- [ ] CLI bridge service setup

### 📅 Next Steps
1. Implement core types and error handling
2. Create basic container list view in Leptos
3. Set up WebSocket communication for real-time updates
4. Integrate with Bolt's container API
5. Add authentication and security

---

## 🎯 Why This Approach?

### Technical Advantages
- **Performance**: WASM + Rust eliminates JavaScript overhead
- **Type Safety**: Full-stack Rust prevents runtime errors
- **Real-time**: WebSocket streams for live container monitoring
- **Modern**: Leptos provides React-like DX with Rust safety

### Gaming Focus
- **GPU First**: Built-in GPU monitoring and management
- **Low Latency**: Optimized for real-time gaming metrics
- **Proton Integration**: First-class Windows game support
- **Performance**: Minimal overhead on gaming workloads

### Developer Experience
- **Single Language**: Rust across frontend and backend
- **Hot Reload**: Fast development iteration
- **Type Driven**: Compile-time error catching
- **Modern Tools**: Cargo, clippy, integrated testing

---

**GhostPanel will be the crown jewel of container management interfaces** - combining the familiarity of Portainer with the power of Rust, the performance of WASM, and the gaming-first philosophy of the Bolt ecosystem.

Let's build the future of container management! 🚀🎮