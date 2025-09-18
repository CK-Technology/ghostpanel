# Container Management in GhostPanel

GhostPanel provides comprehensive container management capabilities through its integration with the Bolt container runtime. This document covers all aspects of container operations, from creation through monitoring and management.

## Table of Contents

1. [Overview](#overview)
2. [Container Creation Wizard](#container-creation-wizard)
3. [Container Management UI](#container-management-ui)
4. [Container Operations](#container-operations)
5. [Gaming Features](#gaming-features)
6. [GPU Allocation](#gpu-allocation)
7. [Performance Monitoring](#performance-monitoring)
8. [API Reference](#api-reference)
9. [Configuration](#configuration)
10. [Troubleshooting](#troubleshooting)

## Overview

GhostPanel's container management system is built around the following key components:

- **Bolt Container Runtime**: High-performance container runtime optimized for gaming workloads
- **Container Creation Wizard**: Step-by-step interface for creating containers with advanced configurations
- **Real-time Monitoring**: Live performance metrics and container status updates
- **Gaming Integration**: Specialized features for gaming containers including Proton support
- **GPU Management**: Direct GPU allocation and monitoring for gaming and compute workloads

### Key Features

- ✅ **Multi-step Container Creation Wizard**
- ✅ **Real-time Container Monitoring** (auto-refresh every 5 seconds)
- ✅ **Gaming Container Support** (Proton, Wine, Steam integration)
- ✅ **GPU Allocation** (NVIDIA/AMD support with isolation)
- ✅ **Advanced Performance Metrics** (CPU, Memory, GPU, Network, Disk I/O)
- ✅ **Container Operations** (Start, Stop, Restart, Remove)
- ✅ **Log Viewing** (Real-time and historical logs)
- ✅ **Registry Integration** (Browse and pull images from configured registries)

## Container Creation Wizard

The container creation wizard provides a guided, 4-step process for creating containers with all necessary configurations.

### Step 1: Image Selection

1. **Search Images**: Search across all configured registries
2. **Browse Results**: View images with metadata (size, registry, tags)
3. **Select Image**: Choose the base image for your container

```
Search: nginx
Results:
- nginx:latest (Docker Hub) - 142.5 MB
- nginx:alpine (Docker Hub) - 23.1 MB
- custom/nginx:v1.0 (Private Registry) - 156.3 MB
```

### Step 2: Basic Configuration

Configure fundamental container settings:

- **Container Name**: Unique identifier for the container
- **Restart Policy**: How the container should restart on failure
  - `No`: Never restart automatically
  - `Always`: Always restart if stopped
  - `Unless Stopped`: Restart unless manually stopped
  - `On Failure`: Only restart on non-zero exit codes
- **Special Features**:
  - **Gaming Mode**: Enables Proton, Wine, and gaming optimizations
  - **GPU Access**: Allocates dedicated GPU resources

### Step 3: Network & Storage

Configure advanced networking and storage options:

#### Port Mappings
Map container ports to host system:
```
Host Port: 8080 → Container Port: 80 (TCP)
Host Port: Auto → Container Port: 443 (TCP)
```

#### Volume Mounts
Bind host directories to container paths:
```
Host Path: /var/app/data → Container Path: /app/data (Read/Write)
Host Path: /etc/config → Container Path: /config (Read Only)
```

#### Environment Variables
Set runtime environment configuration:
```
NODE_ENV=production
DATABASE_URL=postgresql://localhost:5432/mydb
DEBUG=true
```

### Step 4: Review & Create

Final review of all container configuration before creation:

```yaml
Container Summary:
  Name: my-web-app
  Image: nginx:alpine
  Restart Policy: Always
  Features: Standard
  Ports: 8080:80/tcp, auto:443/tcp
  Volumes: 2 volume(s)
  Environment: 3 variable(s)
```

## Container Management UI

The container management interface provides a comprehensive view of all containers with real-time updates.

### Container Cards

Each container is displayed in a detailed card showing:

```
┌─────────────────────────────────────────────────────────────┐
│ my-web-app                    [RUNNING] [GAMING] [GPU]       │
│ nginx:alpine                                                 │
│                                                              │
│ Configuration          Performance                           │
│ Ports: 8080:80        CPU: 15.2%                           │
│ Networks: bridge      Memory: 128MB/512MB (25.0%)          │
│ Proton: 8.0-3         GPU: 45.2% (1.2GB/2GB)              │
│                       FPS: 60                               │
│                                                              │
│ [Stop] [Restart] [Logs] [Details]                          │
└─────────────────────────────────────────────────────────────┘
```

### Status Indicators

- 🟢 **RUNNING**: Container is active and healthy
- 🔴 **STOPPED**: Container has exited
- 🟡 **PAUSED**: Container is paused
- 🔄 **RESTARTING**: Container is restarting
- ⚫ **DEAD**: Container failed to start

### Special Badges

- **GAMING**: Gaming mode enabled with Proton/Wine support
- **GPU**: Dedicated GPU allocation active

### Summary Statistics

Dashboard provides overview metrics:

```
┌─────────────────────────────────────────────────────────────┐
│                         Summary                             │
│                                                             │
│   Total: 12    Running: 8    Stopped: 3    Gaming: 4       │
│                                                             │
│                    GPU Enabled: 6                          │
└─────────────────────────────────────────────────────────────┘
```

## Container Operations

### Start Container
Starts a stopped container with all previous configurations intact.

**API Endpoint**: `POST /api/v1/containers/{id}/start`

### Stop Container
Gracefully stops a running container with optional timeout.

**API Endpoint**: `POST /api/v1/containers/{id}/stop`
```json
{
  "action": "stop",
  "timeout": 30,
  "force": false
}
```

### Restart Container
Combines stop and start operations with configurable timeout.

**API Endpoint**: `POST /api/v1/containers/{id}/restart`

### Remove Container
Permanently deletes a container and optionally its volumes.

**API Endpoint**: `DELETE /api/v1/containers/{id}`
```json
{
  "action": "remove",
  "force": true,
  "remove_volumes": false
}
```

## Gaming Features

GhostPanel includes specialized support for gaming containers with advanced optimizations.

### Proton Integration

Automatic Proton configuration for Windows games on Linux:

```yaml
Gaming Configuration:
  proton_version: "8.0-3"
  wine_version: null
  steam_app_id: 292030  # The Witcher 3
  optimization_profile: "gaming"
```

### Performance Optimizations

Gaming containers automatically receive:

- **CPU Priority**: Higher scheduling priority for game processes
- **Memory Optimization**: Reduced memory fragmentation and improved allocation
- **I/O Scheduling**: Optimized disk and network I/O for gaming workloads
- **GPU Access**: Direct GPU passthrough with minimal overhead

### Steam Integration

Native Steam client support within containers:

- Steam App ID detection and optimization
- Steam Deck compatibility mode
- Automatic controller configuration
- Steam Cloud save synchronization

## GPU Allocation

Advanced GPU management for gaming and compute workloads.

### GPU Configuration

```yaml
GPU Allocation:
  device_id: "gpu0"
  gpu_type: "nvidia"          # nvidia, amd, intel
  memory_mb: 2048            # Dedicated VRAM allocation
  compute_units: 1           # Number of compute units
  isolation_level: "process" # process, container, system
```

### Supported GPU Types

- **NVIDIA**: Full CUDA support with nvidia-docker integration
- **AMD**: ROCm support for compute workloads
- **Intel**: Intel GPU support for integrated graphics

### GPU Monitoring

Real-time GPU metrics displayed in container cards:

```
GPU Metrics:
  Utilization: 45.2%
  Memory: 1.2GB / 2GB (60%)
  Temperature: 72°C
  Power: 180W
```

## Performance Monitoring

Comprehensive performance monitoring with real-time updates.

### CPU Metrics
- **Usage Percentage**: Current CPU utilization
- **Load Average**: System load across multiple time periods
- **Process Count**: Number of active processes

### Memory Metrics
- **Used Memory**: Current memory consumption
- **Memory Limit**: Container memory limit
- **Usage Percentage**: Memory utilization ratio
- **Cache/Buffer**: System cache usage

### GPU Metrics (when allocated)
- **GPU Utilization**: Graphics processing usage
- **Memory Usage**: GPU memory consumption
- **Temperature**: GPU core temperature
- **Power Consumption**: Current power draw

### Network I/O
- **Received Bytes**: Incoming network traffic
- **Transmitted Bytes**: Outgoing network traffic
- **Packet Counts**: Network packet statistics

### Disk I/O
- **Read Operations**: Disk read requests
- **Write Operations**: Disk write requests
- **Bytes Read/Written**: Total disk throughput

### Gaming Metrics (for gaming containers)
- **FPS**: Current frames per second
- **Frame Time**: Average frame render time
- **Input Latency**: Input-to-display latency
- **Network Latency**: Network round-trip time

## API Reference

### Container Management Endpoints

#### List Containers
```http
GET /api/v1/containers
```

**Response**:
```json
{
  "containers": [
    {
      "id": "abc123def456",
      "name": "my-web-app",
      "image": "nginx:alpine",
      "status": "Running",
      "ports": [...],
      "volumes": [...],
      "performance_metrics": {...}
    }
  ]
}
```

#### Create Container
```http
POST /api/v1/containers
```

**Request**:
```json
{
  "name": "my-web-app",
  "image": "nginx:alpine",
  "ports": [
    {
      "container_port": 80,
      "host_port": 8080,
      "protocol": "tcp"
    }
  ],
  "volumes": [
    {
      "source": "/host/path",
      "target": "/container/path",
      "read_only": false
    }
  ],
  "env": {
    "NODE_ENV": "production"
  },
  "restart_policy": "Always",
  "gaming_config": {
    "proton_version": "8.0-3",
    "optimization_profile": "gaming"
  }
}
```

#### Get Container Details
```http
GET /api/v1/containers/{id}
```

#### Container Operations
```http
POST /api/v1/containers/{id}/start
POST /api/v1/containers/{id}/stop
POST /api/v1/containers/{id}/restart
DELETE /api/v1/containers/{id}
```

#### Container Logs
```http
GET /api/v1/containers/{id}/logs
```

#### Container Statistics
```http
GET /api/v1/containers/{id}/stats
```

### Image Search Endpoints

#### Search Images
```http
GET /api/v1/images/search?q=nginx
```

**Response**:
```json
[
  {
    "name": "nginx",
    "tag": "latest",
    "registry_url": "https://registry-1.docker.io",
    "size": 142534144,
    "created": "2024-01-15T10:30:00Z",
    "digest": "sha256:abc123..."
  }
]
```

## Configuration

### Environment Variables

Configure GhostPanel agent behavior:

```bash
# Bolt API Configuration
BOLT_API_URL=bolt://localhost:8080
BOLT_TIMEOUT=30

# Agent Configuration
AGENT_PORT=8000
WEB_PORT=3000

# GPU Configuration
GPU_RUNTIME=nvidia
GPU_ISOLATION=process

# Gaming Configuration
PROTON_PATH=/usr/local/proton
STEAM_RUNTIME=true
```

### Container Runtime Settings

Configure Bolt runtime in `~/.config/bolt/config.toml`:

```toml
[runtime]
root = "/var/lib/bolt"
state = "/run/bolt"
tmp = "/tmp/bolt"

[network]
default_bridge = "bolt0"
enable_ipv6 = true

[gpu]
nvidia_runtime = "/usr/bin/nvidia-container-runtime"
amd_runtime = "/usr/bin/rocm-container-runtime"

[gaming]
proton_versions = ["8.0-3", "7.0-4", "6.3-8"]
wine_versions = ["8.0", "7.22"]
steam_runtime = true
```

## Troubleshooting

### Common Issues

#### Container Creation Fails
```
Error: Failed to create container: image not found
```

**Solution**:
1. Verify image exists in configured registries
2. Check registry authentication
3. Try pulling image manually first

#### GPU Not Available
```
Error: GPU allocation failed: no devices available
```

**Solution**:
1. Verify GPU drivers are installed
2. Check container runtime GPU support
3. Ensure GPU is not already allocated

#### Container Won't Start
```
Error: Container failed to start: port already in use
```

**Solution**:
1. Check port conflicts with other containers
2. Use different host port mapping
3. Stop conflicting services

#### Performance Issues
```
Warning: High memory usage detected
```

**Solution**:
1. Increase container memory limit
2. Optimize application memory usage
3. Check for memory leaks

### Debug Mode

Enable debug logging:

```bash
export RUST_LOG=debug
export BOLT_DEBUG=true
```

### Log Locations

- **Agent Logs**: `/var/log/gpanel/agent.log`
- **Container Logs**: `/var/log/bolt/containers/{id}.log`
- **Runtime Logs**: `/var/log/bolt/runtime.log`

### Health Checks

Verify system health:

```bash
# Check agent status
curl http://localhost:8000/health

# Check container runtime
bolt system info

# Check GPU availability
nvidia-smi  # for NVIDIA GPUs
rocm-smi    # for AMD GPUs
```

### Performance Tuning

#### For Gaming Containers
- Enable CPU governor performance mode
- Increase container memory limits
- Use host networking for reduced latency
- Allocate dedicated GPU resources

#### For Compute Workloads
- Enable huge pages for memory optimization
- Use dedicated CPU cores with cpuset
- Optimize I/O scheduler for workload type
- Configure NUMA affinity for multi-socket systems

## Best Practices

### Container Design
1. **Single Responsibility**: One service per container
2. **Stateless**: Store persistent data in volumes
3. **Health Checks**: Implement proper health endpoints
4. **Resource Limits**: Always set memory and CPU limits

### Security
1. **Non-root User**: Run containers with non-privileged users
2. **Read-only Filesystem**: Use read-only containers when possible
3. **Network Segmentation**: Use custom networks for isolation
4. **Secrets Management**: Use environment variables for sensitive data

### Performance
1. **Image Size**: Use minimal base images (Alpine, distroless)
2. **Layer Optimization**: Minimize Docker layers
3. **Resource Allocation**: Right-size CPU and memory limits
4. **Monitoring**: Implement comprehensive monitoring

### Gaming Specific
1. **GPU Affinity**: Dedicate GPU to single gaming container
2. **Audio**: Configure PulseAudio or ALSA passthrough
3. **Input Devices**: Map game controllers and input devices
4. **Display**: Configure X11 or Wayland display forwarding

---

For additional support and advanced configurations, refer to the [GhostPanel Documentation](https://docs.ghostpanel.io) or join our [Discord Community](https://discord.gg/ghostpanel).