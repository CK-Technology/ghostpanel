# GhostPanel Edge Agents Documentation

GhostPanel Edge Agents are lightweight services deployed across your infrastructure to monitor and manage Bolt containers, system resources, and gaming workloads. They communicate with the central GhostPanel instance via QUIC/HTTP3 for ultra-low latency operations.

## Architecture Overview

```
┌─────────────────┐    QUIC/HTTP3     ┌─────────────────┐
│   GhostPanel    │◄─────────────────►│   Edge Agent    │
│   Central Hub   │                   │    (Node 1)     │
│   Port 9443     │                   │   Port 8000     │
└─────────────────┘                   └─────────────────┘
         │                                      │
         │                                      ▼
         │                            ┌─────────────────┐
         │                            │ Bolt Container  │
         │                            │    Runtime      │
         │                            └─────────────────┘
         │
         │         QUIC/HTTP3     ┌─────────────────┐
         └─────────────────────►│   Edge Agent    │
                                │    (Node 2)     │
                                │   Port 8000     │
                                └─────────────────┘
```

## Edge Agent Components

### 1. System Monitor
- **CPU Usage**: Real-time CPU utilization per core
- **Memory**: RAM usage, swap, available memory
- **Disk I/O**: Read/write speeds, disk utilization
- **Network**: Bandwidth usage, packet statistics
- **GPU**: GPU utilization, memory, temperature (if available)

### 2. Container Manager
- **Container Operations**: Start, stop, restart, remove
- **Image Management**: Pull, remove, inspect images
- **Resource Monitoring**: Container-specific resource usage
- **Health Checks**: Container health status and diagnostics

### 3. Gaming Optimizer
- **GPU Allocation**: Dynamic GPU resource assignment
- **Proton Management**: Steam Proton compatibility layer
- **Performance Monitoring**: FPS, latency, frame times
- **Audio/Video Streaming**: Remote gaming session support

### 4. Security Monitor
- **Container Isolation**: Runtime security monitoring
- **Network Security**: Traffic analysis and filtering
- **Resource Limits**: Enforce container resource constraints
- **Audit Logging**: Security event logging

## Installation

### Prerequisites

```bash
# Ensure Bolt is installed
bolt version

# Install required dependencies
sudo apt update
sudo apt install -y curl wget systemd

# For GPU nodes (optional)
sudo apt install -y nvidia-docker2 nvidia-container-runtime
```

### Download and Install

```bash
# Download latest edge agent
curl -fsSL https://releases.ghostpanel.dev/agent/install.sh | bash

# Or manual installation
wget https://releases.ghostpanel.dev/agent/v0.1.0/gpanel-agent-linux-amd64
chmod +x gpanel-agent-linux-amd64
sudo mv gpanel-agent-linux-amd64 /usr/local/bin/gpanel-agent
```

### Service Installation

```bash
# Create system service
sudo tee /etc/systemd/system/gpanel-agent.service > /dev/null <<EOF
[Unit]
Description=GhostPanel Edge Agent
After=network.target bolt.service
Wants=network.target

[Service]
Type=exec
User=root
Group=root
ExecStart=/usr/local/bin/gpanel-agent
Restart=always
RestartSec=5
Environment=GPANEL_AGENT_CONFIG=/etc/gpanel/agent.toml

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable gpanel-agent
sudo systemctl start gpanel-agent
```

## Configuration

### Agent Configuration File

Create `/etc/gpanel/agent.toml`:

```toml
[agent]
# Agent identification
node_id = "edge-node-01"
node_name = "Gaming Node 1"
node_type = "gaming"  # Options: "gaming", "compute", "storage", "general"

# Network configuration
bind_address = "0.0.0.0:8000"
public_address = "192.168.1.100:8000"

# Central hub connection
central_hub = "https://ghostpanel.example.com:9443"
auth_token = "${GPANEL_AGENT_TOKEN}"

# TLS configuration
tls_enabled = true
tls_cert_path = "/etc/gpanel/certs/agent.crt"
tls_key_path = "/etc/gpanel/certs/agent.key"
tls_ca_path = "/etc/gpanel/certs/ca.crt"

# QUIC settings
[quic]
enabled = true
max_connections = 100
idle_timeout = 300
keep_alive_interval = 30

# Monitoring configuration
[monitoring]
interval = 5  # seconds
cpu_detailed = true
memory_detailed = true
disk_detailed = true
network_detailed = true
gpu_enabled = true

# Container runtime
[runtime]
socket_path = "/var/run/bolt.sock"
image_cache_size = "10GB"
container_log_max_size = "100MB"
container_log_max_files = 5

# Gaming-specific settings
[gaming]
enabled = true
gpu_sharing = false
proton_auto_install = true
proton_versions = ["8.0-5", "7.0-6", "6.3-8"]
vnc_enabled = true
vnc_port_range = "5900-5999"
audio_enabled = true
streaming_quality = "high"  # Options: "low", "medium", "high", "ultra"

# Security settings
[security]
enable_seccomp = true
enable_apparmor = true
enable_selinux = false
container_user_namespace = true
readonly_root_filesystem = false

# Logging
[logging]
level = "info"  # Options: "trace", "debug", "info", "warn", "error"
format = "json"
output = "stdout"
file_path = "/var/log/gpanel-agent.log"
max_file_size = "100MB"
max_files = 10
```

### Environment Variables

```bash
# Required
export GPANEL_AGENT_TOKEN="your-agent-authentication-token"

# Optional
export GPANEL_AGENT_CONFIG="/etc/gpanel/agent.toml"
export GPANEL_LOG_LEVEL="info"
export GPANEL_QUIC_ENABLED="true"
export GPANEL_GPU_ENABLED="true"
```

### Authentication Setup

#### Generate Agent Token

On your central GhostPanel instance:

```bash
# Generate a new agent token
gpanel-cli agent token create --name "edge-node-01" --type "gaming" --expires "30d"

# Example output:
# Agent Token: gpanel_agent_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
# Node ID: edge-node-01
# Expires: 2024-02-01T00:00:00Z
```

#### Certificate-Based Authentication (Recommended)

```bash
# Generate agent certificates
mkdir -p /etc/gpanel/certs

# Create certificate signing request
openssl req -new -newkey rsa:4096 -nodes \
    -keyout /etc/gpanel/certs/agent.key \
    -out /etc/gpanel/certs/agent.csr \
    -subj "/CN=edge-node-01.ghostpanel.local"

# Submit CSR to central hub (or use CA)
gpanel-cli cert sign --csr /etc/gpanel/certs/agent.csr \
    --output /etc/gpanel/certs/agent.crt

# Download CA certificate
gpanel-cli cert ca --output /etc/gpanel/certs/ca.crt

# Set proper permissions
chmod 600 /etc/gpanel/certs/agent.key
chmod 644 /etc/gpanel/certs/agent.crt
chmod 644 /etc/gpanel/certs/ca.crt
```

## Agent Operations

### Service Management

```bash
# Check agent status
sudo systemctl status gpanel-agent

# View logs
sudo journalctl -u gpanel-agent -f

# Restart agent
sudo systemctl restart gpanel-agent

# Stop agent
sudo systemctl stop gpanel-agent

# Update configuration
sudo nano /etc/gpanel/agent.toml
sudo systemctl reload gpanel-agent
```

### Health Checks

```bash
# Check agent health
curl https://localhost:8000/health

# Example response:
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime": "24h30m15s",
  "node_id": "edge-node-01",
  "connected_to_hub": true,
  "containers_running": 5,
  "system_load": {
    "cpu_percent": 25.5,
    "memory_percent": 45.2,
    "disk_percent": 62.1
  }
}
```

### Agent Metrics

```bash
# View detailed metrics
curl https://localhost:8000/metrics

# Container-specific metrics
curl https://localhost:8000/containers/metrics

# Gaming session metrics
curl https://localhost:8000/gaming/metrics

# System metrics
curl https://localhost:8000/system/metrics
```

## Gaming Features

### GPU Management

The edge agent automatically detects and manages GPU resources:

```toml
[gaming.gpu]
auto_detect = true
devices = ["/dev/dri/card0", "/dev/dri/renderD128"]
memory_limit = "8GB"
sharing_enabled = false
priority_scheduling = true

[gaming.gpu.allocation]
strategy = "first_available"  # Options: "first_available", "least_used", "round_robin"
reservation_timeout = "30m"
```

### Proton Integration

Automatic Proton version management:

```bash
# List available Proton versions
gpanel-agent gaming proton list

# Install specific Proton version
gpanel-agent gaming proton install 8.0-5

# Set default Proton version
gpanel-agent gaming proton set-default 8.0-5

# Check Proton compatibility
gpanel-agent gaming proton check-game 12345
```

### Gaming Session Management

```bash
# Start gaming session
curl -X POST https://localhost:8000/gaming/sessions \
  -H "Content-Type: application/json" \
  -d '{
    "game": "steam",
    "steam_app_id": "12345",
    "proton_version": "8.0-5",
    "gpu_required": true,
    "resolution": "1920x1080"
  }'

# Monitor session performance
curl https://localhost:8000/gaming/sessions/session_id/performance

# Stream session (WebRTC)
curl https://localhost:8000/gaming/sessions/session_id/stream
```

## API Endpoints

### Agent Management

```http
GET /health
GET /info
GET /metrics
POST /shutdown
POST /restart
```

### Container Operations

```http
GET /containers
POST /containers
GET /containers/{id}
POST /containers/{id}/start
POST /containers/{id}/stop
DELETE /containers/{id}
GET /containers/{id}/logs
GET /containers/{id}/stats
```

### Image Management

```http
GET /images
POST /images/pull
DELETE /images/{id}
GET /images/{id}/inspect
```

### System Monitoring

```http
GET /system/info
GET /system/stats
GET /system/processes
GET /system/resources
```

### Gaming Operations

```http
GET /gaming/sessions
POST /gaming/sessions
GET /gaming/sessions/{id}
DELETE /gaming/sessions/{id}
GET /gaming/templates
GET /gaming/proton/versions
```

## Troubleshooting

### Common Issues

#### Agent Not Connecting to Hub

```bash
# Check network connectivity
curl -k https://ghostpanel.example.com:9443/health

# Verify authentication token
gpanel-cli agent token verify

# Check TLS certificates
openssl x509 -in /etc/gpanel/certs/agent.crt -text -noout

# Test QUIC connectivity
gpanel-agent test-connection --hub https://ghostpanel.example.com:9443
```

#### High Resource Usage

```bash
# Check resource limits
gpanel-agent config show | grep -A 10 "\[resources\]"

# Monitor container resource usage
gpanel-agent containers stats --all

# Check system resources
gpanel-agent system resources --detailed
```

#### Gaming Session Failures

```bash
# Check GPU availability
nvidia-smi  # For NVIDIA GPUs
lspci | grep VGA  # General GPU detection

# Verify Proton installation
gpanel-agent gaming proton verify

# Check gaming session logs
gpanel-agent gaming sessions logs session_id

# Test VNC connectivity
vncviewer localhost:5901
```

### Log Analysis

```bash
# Enable debug logging
sudo tee -a /etc/gpanel/agent.toml > /dev/null <<EOF
[logging]
level = "debug"
EOF

sudo systemctl restart gpanel-agent

# View specific component logs
sudo journalctl -u gpanel-agent | grep "component=container"
sudo journalctl -u gpanel-agent | grep "component=gaming"
sudo journalctl -u gpanel-agent | grep "component=quic"
```

### Performance Tuning

#### System Optimization

```bash
# Increase file descriptor limits
echo "gpanel-agent soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "gpanel-agent hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Optimize network settings
sudo tee -a /etc/sysctl.conf > /dev/null <<EOF
net.core.rmem_max = 16777216
net.core.wmem_max = 16777216
net.ipv4.tcp_rmem = 4096 87380 16777216
net.ipv4.tcp_wmem = 4096 65536 16777216
net.core.netdev_max_backlog = 5000
EOF

sudo sysctl -p
```

#### QUIC Tuning

```toml
[quic.performance]
send_buffer_size = "1MB"
receive_buffer_size = "1MB"
max_concurrent_streams = 1000
connection_pool_size = 50
congestion_control = "bbr"  # Options: "cubic", "bbr", "reno"
```

## Deployment Strategies

### Single Node Deployment

```yaml
# docker-compose.yml for edge agent
version: '3.8'
services:
  gpanel-agent:
    image: ghostpanel/agent:latest
    container_name: gpanel-agent
    restart: unless-stopped
    privileged: true
    network_mode: host
    volumes:
      - /var/run/bolt.sock:/var/run/bolt.sock
      - /etc/gpanel:/etc/gpanel:ro
      - /var/log/gpanel:/var/log/gpanel
      - /dev:/dev
    environment:
      - GPANEL_AGENT_TOKEN=${GPANEL_AGENT_TOKEN}
      - GPANEL_LOG_LEVEL=info
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: DaemonSet
metadata:
  name: gpanel-agent
  namespace: gpanel-system
spec:
  selector:
    matchLabels:
      app: gpanel-agent
  template:
    metadata:
      labels:
        app: gpanel-agent
    spec:
      hostNetwork: true
      hostPID: true
      containers:
      - name: gpanel-agent
        image: ghostpanel/agent:latest
        securityContext:
          privileged: true
        env:
        - name: GPANEL_AGENT_TOKEN
          valueFrom:
            secretKeyRef:
              name: gpanel-agent-token
              key: token
        - name: NODE_NAME
          valueFrom:
            fieldRef:
              fieldPath: spec.nodeName
        volumeMounts:
        - name: bolt-socket
          mountPath: /var/run/bolt.sock
        - name: config
          mountPath: /etc/gpanel
        - name: dev
          mountPath: /dev
        ports:
        - containerPort: 8000
          hostPort: 8000
      volumes:
      - name: bolt-socket
        hostPath:
          path: /var/run/bolt.sock
      - name: config
        configMap:
          name: gpanel-agent-config
      - name: dev
        hostPath:
          path: /dev
```

### Ansible Playbook

```yaml
---
- name: Deploy GhostPanel Edge Agent
  hosts: edge_nodes
  become: yes
  vars:
    gpanel_version: "0.1.0"
    gpanel_hub_url: "https://ghostpanel.example.com:9443"

  tasks:
    - name: Download GhostPanel Agent
      get_url:
        url: "https://releases.ghostpanel.dev/agent/v{{ gpanel_version }}/gpanel-agent-linux-amd64"
        dest: /usr/local/bin/gpanel-agent
        mode: '0755'

    - name: Create gpanel directory
      file:
        path: /etc/gpanel
        state: directory
        mode: '0755'

    - name: Template agent configuration
      template:
        src: agent.toml.j2
        dest: /etc/gpanel/agent.toml
        mode: '0644'
      notify: restart gpanel-agent

    - name: Install systemd service
      template:
        src: gpanel-agent.service.j2
        dest: /etc/systemd/system/gpanel-agent.service
      notify:
        - reload systemd
        - restart gpanel-agent

    - name: Start and enable service
      systemd:
        name: gpanel-agent
        state: started
        enabled: yes

  handlers:
    - name: reload systemd
      systemd:
        daemon_reload: yes

    - name: restart gpanel-agent
      systemd:
        name: gpanel-agent
        state: restarted
```

## Security Considerations

### Network Security

```toml
[security.network]
# Allow only specific IPs to connect
allowed_ips = ["192.168.1.0/24", "10.0.0.0/8"]

# Enable mTLS for all connections
require_mtls = true

# Certificate pinning
pin_central_hub_cert = true
pinned_cert_hash = "sha256:1234567890abcdef..."

# Rate limiting
rate_limit_requests_per_minute = 1000
rate_limit_burst = 100
```

### Container Security

```toml
[security.containers]
# Run containers as non-root by default
default_user = "1000:1000"

# Enable container security profiles
enable_seccomp = true
seccomp_profile = "/etc/gpanel/seccomp.json"

enable_apparmor = true
apparmor_profile = "gpanel-container"

# Restrict capabilities
allowed_capabilities = ["CHOWN", "DAC_OVERRIDE", "SETUID", "SETGID"]
dropped_capabilities = ["SYS_ADMIN", "NET_ADMIN", "SYS_TIME"]

# Read-only root filesystem by default
readonly_root_filesystem = true
```

### Audit Logging

```toml
[security.audit]
enabled = true
log_level = "info"
log_format = "json"
log_destination = "/var/log/audit/gpanel-agent.log"

# Events to log
log_container_operations = true
log_image_operations = true
log_network_connections = true
log_authentication_events = true
log_configuration_changes = true

# Log rotation
max_log_size = "100MB"
max_log_files = 10
compress_old_logs = true
```

## Monitoring and Alerting

### Prometheus Integration

```toml
[monitoring.prometheus]
enabled = true
listen_address = "0.0.0.0:9090"
metrics_path = "/metrics"

# Custom metrics
[monitoring.custom_metrics]
container_startup_time = true
gaming_session_fps = true
gpu_utilization = true
network_latency_to_hub = true
```

### Grafana Dashboards

Pre-built dashboards available:
- **Node Overview**: System resources, container counts, network usage
- **Gaming Performance**: FPS, latency, GPU utilization
- **Container Metrics**: Resource usage per container
- **Security Events**: Authentication, access violations

### Alerting Rules

```yaml
# alerting.yml
groups:
- name: gpanel-agent
  rules:
  - alert: AgentDown
    expr: up{job="gpanel-agent"} == 0
    for: 5m
    annotations:
      summary: "GhostPanel agent is down"

  - alert: HighCPUUsage
    expr: cpu_usage_percent > 90
    for: 10m
    annotations:
      summary: "High CPU usage on {{ $labels.node_name }}"

  - alert: GameSessionFPSLow
    expr: gaming_session_fps < 30
    for: 2m
    annotations:
      summary: "Low FPS in gaming session {{ $labels.session_id }}"
```

## Additional Resources

- [GhostPanel Central Hub Documentation](./API.md)
- [QUIC Socket Proxy Documentation](./SOCKET_PROXY.md)
- [Authentication Setup](./SSO.md)
- [Bolt Container Runtime](https://bolt-containers.dev)
- [Agent Configuration Examples](https://github.com/ghostpanel/examples)
- [Community Support](https://discord.gg/ghostpanel)