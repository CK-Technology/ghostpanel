# GhostPanel Socket Proxy Documentation

The GhostPanel Socket Proxy is a high-performance QUIC/HTTP3-based proxy service that provides ultra-low latency communication between GhostPanel components and Bolt containers. It serves as the central networking hub for the entire GhostPanel ecosystem.

## Architecture Overview

```
                    ┌─────────────────┐
                    │   Web Browser   │
                    │  (Frontend UI)  │
                    └─────────┬───────┘
                              │ HTTPS/WSS
                              ▼
┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐
│   Edge Agent    │  │ Socket Proxy    │  │  Bolt Runtime   │
│   Port 8000     │◄─┤   Port 9443     ├─►│   Port 8080     │
└─────────────────┘  │   (QUIC/HTTP3)  │  └─────────────────┘
                     └─────────┬───────┘
                               │ QUIC
                               ▼
                     ┌─────────────────┐
                     │ Agent Service   │
                     │   Port 8000     │
                     └─────────────────┘
```

## Key Features

### 🚀 QUIC/HTTP3 Protocol Stack
- **Ultra-low latency**: ~50% reduction in connection time vs HTTP/2
- **Multiplexing**: Handle thousands of concurrent streams
- **0-RTT connections**: Instant reconnections for known clients
- **Built-in encryption**: TLS 1.3 by default

### 🔄 Smart Request Routing
- **Path-based routing**: Route requests based on URL patterns
- **Service discovery**: Automatic backend service detection
- **Load balancing**: Distribute load across multiple backends
- **Failover**: Automatic fallback from QUIC to HTTP/1.1

### 📊 Real-time Metrics
- **Connection statistics**: Active connections, request counts
- **Performance metrics**: Latency, throughput, error rates
- **Resource monitoring**: CPU, memory, network usage
- **Gaming optimizations**: FPS tracking, latency monitoring

### 🎮 Gaming Optimizations
- **Container-aware multiplexing**: Prioritize gaming traffic
- **GPU stream handling**: Optimized for GPU workload data
- **Real-time dashboard**: Live performance monitoring
- **Low-latency streaming**: WebRTC integration for game streaming

## Installation and Setup

### Prerequisites

```bash
# Ensure dependencies are installed
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/ghostpanel/ghostpanel.git
cd ghostpanel

# Build the proxy service
cargo build --release --bin gpanel-proxy

# Install binary
sudo cp target/release/gpanel-proxy /usr/local/bin/
```

### Docker Installation

```bash
# Pull the official image
docker pull ghostpanel/proxy:latest

# Or build locally
docker build -t ghostpanel/proxy -f crates/gpanel-proxy/Dockerfile .
```

## Configuration

### Command Line Options

```bash
gpanel-proxy --help

GhostPanel QUIC Proxy - QUIC-based socket proxy for GhostPanel edge networking

USAGE:
    gpanel-proxy [OPTIONS]

OPTIONS:
        --quic-addr <QUIC_ADDR>
            QUIC server bind address [default: 0.0.0.0:9443]

        --http-addr <HTTP_ADDR>
            HTTP/1.1 fallback server bind address [default: 0.0.0.0:9080]

        --bolt-api <BOLT_API>
            Target Bolt API endpoint [default: bolt://localhost:8080]

        --cert-path <CERT_PATH>
            TLS certificate path (optional, generates self-signed in dev)

        --key-path <KEY_PATH>
            TLS private key path (optional)

        --dev-mode
            Enable development mode (allows insecure connections)

        --max-connections <MAX_CONNECTIONS>
            Maximum concurrent connections [default: 1000]

        --idle-timeout <IDLE_TIMEOUT>
            Connection idle timeout in seconds [default: 300]

    -h, --help
            Print help information
```

### Configuration File

Create `/etc/gpanel/proxy.toml`:

```toml
[proxy]
# Server binding
quic_addr = "0.0.0.0:9443"
http_addr = "0.0.0.0:9080"

# Target services
bolt_api_url = "bolt://localhost:8080"
agent_api_url = "http://localhost:8000"

# TLS configuration
cert_path = "/etc/gpanel/certs/proxy.crt"
key_path = "/etc/gpanel/certs/proxy.key"
ca_path = "/etc/gpanel/certs/ca.crt"

# Connection limits
max_connections = 1000
idle_timeout = 300  # seconds
keep_alive_interval = 30  # seconds

# Development mode (disable in production)
dev_mode = false
allow_insecure = false

[quic]
# QUIC-specific settings
enabled = true
http3_enabled = true
max_concurrent_streams = 1000
initial_max_data = 10485760  # 10MB
initial_max_stream_data = 1048576  # 1MB
max_ack_delay = 25  # milliseconds

# Performance tuning
send_buffer_size = 1048576  # 1MB
receive_buffer_size = 1048576  # 1MB
congestion_control = "bbr"  # Options: cubic, bbr, reno

[routing]
# Request routing configuration
[routing.rules]
# Container operations route to Bolt API
"/api/containers/*" = "bolt"
"/api/images/*" = "bolt"
"/api/networks/*" = "bolt"
"/api/volumes/*" = "bolt"

# System stats route to agent
"/api/system/stats" = "agent"
"/api/system/info" = "agent"

# Gaming operations route to agent
"/api/gaming/*" = "agent"

# Proxy stats (handled internally)
"/api/stats" = "internal"

# Static files (handled internally)
"/*" = "static"

[backends]
# Backend service definitions
[backends.bolt]
url = "bolt://localhost:8080"
protocol = "quic"  # quic, http, auto
health_check = "/health"
timeout = 30
retries = 3

[backends.agent]
url = "http://localhost:8000"
protocol = "http"
health_check = "/health"
timeout = 15
retries = 2

[security]
# Security settings
enable_cors = true
cors_allowed_origins = ["https://localhost:9443", "https://*.ghostpanel.local"]
cors_allowed_methods = ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
cors_allowed_headers = ["Authorization", "Content-Type", "X-Requested-With"]

# Rate limiting
enable_rate_limiting = true
rate_limit_requests_per_minute = 1000
rate_limit_burst = 100

# Authentication
require_auth = true
auth_header = "Authorization"
auth_prefix = "Bearer "

[logging]
level = "info"  # trace, debug, info, warn, error
format = "json"
output = "stdout"
file_path = "/var/log/gpanel-proxy.log"

[metrics]
# Metrics and monitoring
enabled = true
listen_addr = "127.0.0.1:9090"
path = "/metrics"
update_interval = 5  # seconds

# Custom metrics
track_request_duration = true
track_connection_count = true
track_error_rates = true
track_gaming_performance = true
```

### Environment Variables

```bash
# Basic configuration
export GPANEL_PROXY_QUIC_ADDR="0.0.0.0:9443"
export GPANEL_PROXY_HTTP_ADDR="0.0.0.0:9080"
export GPANEL_PROXY_BOLT_API="bolt://localhost:8080"

# TLS certificates
export GPANEL_PROXY_CERT_PATH="/etc/gpanel/certs/proxy.crt"
export GPANEL_PROXY_KEY_PATH="/etc/gpanel/certs/proxy.key"

# Performance settings
export GPANEL_PROXY_MAX_CONNECTIONS="1000"
export GPANEL_PROXY_IDLE_TIMEOUT="300"

# Development mode
export GPANEL_PROXY_DEV_MODE="false"

# Logging
export RUST_LOG="gpanel_proxy=info,quinn=warn"
export GPANEL_LOG_FORMAT="json"
```

## Service Deployment

### Systemd Service

Create `/etc/systemd/system/gpanel-proxy.service`:

```ini
[Unit]
Description=GhostPanel Socket Proxy
After=network.target
Wants=network.target

[Service]
Type=exec
User=gpanel
Group=gpanel
ExecStart=/usr/local/bin/gpanel-proxy
Restart=always
RestartSec=5
LimitNOFILE=65536
EnvironmentFile=/etc/gpanel/proxy.env

# Security hardening
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=yes
ReadWritePaths=/var/log/gpanel /var/run/gpanel
PrivateTmp=yes
ProtectKernelTunables=yes
ProtectControlGroups=yes
RestrictRealtime=yes

[Install]
WantedBy=multi-user.target
```

### Docker Compose

```yaml
version: '3.8'

services:
  gpanel-proxy:
    image: ghostpanel/proxy:latest
    container_name: gpanel-proxy
    restart: unless-stopped
    ports:
      - "9443:9443"  # QUIC/HTTP3
      - "9080:9080"  # HTTP/1.1 fallback
      - "9090:9090"  # Metrics
    volumes:
      - ./config/proxy.toml:/etc/gpanel/proxy.toml:ro
      - ./certs:/etc/gpanel/certs:ro
      - ./logs:/var/log/gpanel
    environment:
      - RUST_LOG=gpanel_proxy=info
      - GPANEL_PROXY_CONFIG=/etc/gpanel/proxy.toml
    networks:
      - gpanel-network
    depends_on:
      - bolt-daemon
      - gpanel-agent

  bolt-daemon:
    image: bolt/daemon:latest
    container_name: bolt-daemon
    restart: unless-stopped
    privileged: true
    ports:
      - "8080:8080"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - bolt-data:/var/lib/bolt
    networks:
      - gpanel-network

  gpanel-agent:
    image: ghostpanel/agent:latest
    container_name: gpanel-agent
    restart: unless-stopped
    ports:
      - "8000:8000"
    volumes:
      - ./config/agent.toml:/etc/gpanel/agent.toml:ro
      - /var/run/docker.sock:/var/run/docker.sock
    networks:
      - gpanel-network

volumes:
  bolt-data:

networks:
  gpanel-network:
    driver: bridge
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: gpanel-proxy
  namespace: gpanel-system
spec:
  replicas: 3
  selector:
    matchLabels:
      app: gpanel-proxy
  template:
    metadata:
      labels:
        app: gpanel-proxy
    spec:
      containers:
      - name: gpanel-proxy
        image: ghostpanel/proxy:latest
        ports:
        - containerPort: 9443
          name: quic
          protocol: UDP
        - containerPort: 9080
          name: http
          protocol: TCP
        - containerPort: 9090
          name: metrics
          protocol: TCP
        env:
        - name: GPANEL_PROXY_CONFIG
          value: "/etc/gpanel/proxy.toml"
        - name: RUST_LOG
          value: "gpanel_proxy=info"
        volumeMounts:
        - name: config
          mountPath: /etc/gpanel
          readOnly: true
        - name: certs
          mountPath: /etc/gpanel/certs
          readOnly: true
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 9090
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 9090
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: gpanel-proxy-config
      - name: certs
        secret:
          secretName: gpanel-proxy-certs

---
apiVersion: v1
kind: Service
metadata:
  name: gpanel-proxy
  namespace: gpanel-system
spec:
  selector:
    app: gpanel-proxy
  ports:
  - name: quic
    port: 9443
    targetPort: 9443
    protocol: UDP
  - name: http
    port: 9080
    targetPort: 9080
    protocol: TCP
  - name: metrics
    port: 9090
    targetPort: 9090
    protocol: TCP
  type: LoadBalancer
```

## TLS Certificate Management

### Self-Signed Certificates (Development)

```bash
# Generate CA private key
openssl genrsa -out ca.key 4096

# Generate CA certificate
openssl req -new -x509 -key ca.key -sha256 -subj "/C=US/ST=CA/O=GhostPanel/CN=GhostPanel CA" -days 3650 -out ca.crt

# Generate proxy private key
openssl genrsa -out proxy.key 4096

# Create certificate signing request
openssl req -new -key proxy.key -out proxy.csr -subj "/C=US/ST=CA/O=GhostPanel/CN=localhost"

# Generate proxy certificate
openssl x509 -req -in proxy.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out proxy.crt -days 365 -sha256 -extensions v3_req -extfile <(
cat <<EOF
[v3_req]
basicConstraints = CA:FALSE
keyUsage = nonRepudiation, digitalSignature, keyEncipherment
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
DNS.2 = *.ghostpanel.local
IP.1 = 127.0.0.1
IP.2 = ::1
EOF
)

# Set proper permissions
chmod 600 proxy.key ca.key
chmod 644 proxy.crt ca.crt
```

### Let's Encrypt Integration

```bash
# Install certbot
sudo apt install -y certbot

# Generate certificate
sudo certbot certonly --standalone \
  -d ghostpanel.example.com \
  --email admin@example.com \
  --agree-tos \
  --non-interactive

# Copy certificates
sudo cp /etc/letsencrypt/live/ghostpanel.example.com/fullchain.pem /etc/gpanel/certs/proxy.crt
sudo cp /etc/letsencrypt/live/ghostpanel.example.com/privkey.pem /etc/gpanel/certs/proxy.key

# Set up automatic renewal
sudo tee /etc/cron.d/ghostpanel-cert-renewal > /dev/null <<EOF
0 3 * * * root /usr/bin/certbot renew --quiet --post-hook "systemctl reload gpanel-proxy"
EOF
```

## Request Routing

### Built-in Routing Rules

The proxy includes intelligent routing based on request paths:

```rust
// Container operations → Bolt API
"/api/containers/*" → bolt://localhost:8080
"/api/images/*" → bolt://localhost:8080
"/api/networks/*" → bolt://localhost:8080
"/api/volumes/*" → bolt://localhost:8080

// System information → Agent Service
"/api/system/stats" → http://localhost:8000
"/api/system/info" → http://localhost:8000

// Gaming operations → Agent Service
"/api/gaming/*" → http://localhost:8000

// Proxy statistics → Internal handler
"/api/stats" → internal

// Static files → Internal handler
"/" → static file handler
"/assets/*" → static file handler
```

### Custom Routing Configuration

```toml
[routing.custom]
# Add custom routing rules
"/api/v2/containers/*" = "bolt"
"/api/monitoring/*" = "prometheus"
"/api/logs/*" = "loki"

[backends.prometheus]
url = "http://prometheus:9090"
protocol = "http"
health_check = "/-/healthy"

[backends.loki]
url = "http://loki:3100"
protocol = "http"
health_check = "/ready"
```

### Load Balancing

```toml
[routing.load_balancing]
strategy = "round_robin"  # Options: round_robin, least_connections, weighted

# Multiple backend instances
[backends.bolt_cluster]
instances = [
    "bolt://bolt-1:8080",
    "bolt://bolt-2:8080",
    "bolt://bolt-3:8080"
]
health_check_interval = 30
failover_enabled = true
```

## Performance Tuning

### QUIC Optimization

```toml
[quic.performance]
# Buffer sizes
initial_max_data = 10485760  # 10MB
initial_max_stream_data = 1048576  # 1MB
initial_max_streams_bidi = 100
initial_max_streams_uni = 100

# Congestion control
congestion_control_algorithm = "bbr"  # bbr, cubic, reno
initial_congestion_window = 32
min_congestion_window = 4
max_congestion_window = 1000

# Flow control
flow_control_window = 1048576  # 1MB
connection_data_window = 10485760  # 10MB

# Keep-alive
keep_alive_interval = 30000  # 30 seconds
max_idle_timeout = 300000  # 5 minutes

# Recovery
max_ack_delay = 25  # milliseconds
ack_delay_exponent = 3
```

### System Optimization

```bash
# Increase file descriptor limits
echo "gpanel soft nofile 65536" | sudo tee -a /etc/security/limits.conf
echo "gpanel hard nofile 65536" | sudo tee -a /etc/security/limits.conf

# Network buffer tuning
sudo tee -a /etc/sysctl.conf > /dev/null <<EOF
# Increase network buffer sizes
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.core.rmem_default = 65536
net.core.wmem_default = 65536

# Increase max connections
net.core.somaxconn = 65535
net.core.netdev_max_backlog = 5000

# Enable BBR congestion control
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr

# Optimize for low latency
net.ipv4.tcp_low_latency = 1
net.ipv4.tcp_timestamps = 1
net.ipv4.tcp_sack = 1
EOF

sudo sysctl -p
```

### Memory Optimization

```toml
[proxy.memory]
# Connection pool settings
connection_pool_size = 1000
connection_pool_timeout = 30

# Request buffer settings
request_buffer_size = 65536  # 64KB
response_buffer_size = 65536  # 64KB
max_request_size = 10485760  # 10MB

# Cache settings
enable_response_cache = true
cache_size = 104857600  # 100MB
cache_ttl = 300  # 5 minutes
```

## Monitoring and Metrics

### Built-in Metrics

The proxy exposes Prometheus-compatible metrics on `/metrics`:

```prometheus
# Connection metrics
gpanel_proxy_active_connections_total
gpanel_proxy_total_connections_total
gpanel_proxy_failed_connections_total

# Request metrics
gpanel_proxy_requests_total{method, path, status}
gpanel_proxy_request_duration_seconds{method, path}
gpanel_proxy_request_size_bytes{method, path}
gpanel_proxy_response_size_bytes{method, path}

# QUIC metrics
gpanel_proxy_quic_connections_total
gpanel_proxy_quic_streams_total
gpanel_proxy_quic_packets_sent_total
gpanel_proxy_quic_packets_received_total
gpanel_proxy_quic_bytes_sent_total
gpanel_proxy_quic_bytes_received_total

# Backend metrics
gpanel_proxy_backend_requests_total{backend}
gpanel_proxy_backend_failures_total{backend}
gpanel_proxy_backend_response_time_seconds{backend}

# Gaming metrics
gpanel_proxy_gaming_sessions_active
gpanel_proxy_gaming_fps_average
gpanel_proxy_gaming_latency_ms
```

### Health Checks

```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime": "24h30m15s",
  "active_connections": 42,
  "backends": {
    "bolt": {
      "status": "healthy",
      "last_check": "2024-01-01T12:00:00Z",
      "response_time": "15ms"
    },
    "agent": {
      "status": "healthy",
      "last_check": "2024-01-01T12:00:00Z",
      "response_time": "8ms"
    }
  }
}
```

### Readiness Check

```http
GET /ready
```

**Response:**
```json
{
  "ready": true,
  "checks": {
    "quic_server": "ok",
    "http_server": "ok",
    "bolt_backend": "ok",
    "agent_backend": "ok",
    "tls_certificates": "ok"
  }
}
```

### Debug Endpoints

```http
# Connection information
GET /debug/connections

# Configuration dump
GET /debug/config

# Performance statistics
GET /debug/stats

# QUIC connection details
GET /debug/quic
```

## Troubleshooting

### Common Issues

#### QUIC Connection Failures

```bash
# Check QUIC connectivity
curl -3 --http3-only https://localhost:9443/health

# Test with verbose output
curl -3 --http3-only -v https://localhost:9443/api/stats

# Check for firewall issues
sudo ufw status
sudo iptables -L | grep 9443
```

#### TLS Certificate Issues

```bash
# Verify certificate validity
openssl x509 -in /etc/gpanel/certs/proxy.crt -text -noout

# Check certificate chain
openssl verify -CAfile /etc/gpanel/certs/ca.crt /etc/gpanel/certs/proxy.crt

# Test TLS connection
openssl s_client -connect localhost:9443 -servername localhost
```

#### Backend Connection Issues

```bash
# Test Bolt API connectivity
curl http://localhost:8080/health

# Test Agent API connectivity
curl http://localhost:8000/health

# Check proxy logs for backend errors
sudo journalctl -u gpanel-proxy | grep -i "backend\|bolt\|agent"
```

#### Performance Issues

```bash
# Check connection limits
ulimit -n

# Monitor resource usage
htop
iotop
netstat -tulpn | grep 9443

# Check QUIC performance
gpanel-proxy debug quic-stats
```

### Log Analysis

```bash
# Enable debug logging
export RUST_LOG="gpanel_proxy=debug,quinn=debug"

# Filter specific components
sudo journalctl -u gpanel-proxy | grep "component=quic"
sudo journalctl -u gpanel-proxy | grep "component=routing"
sudo journalctl -u gpanel-proxy | grep "component=backend"

# Real-time log monitoring
sudo journalctl -u gpanel-proxy -f --output=json | jq '.MESSAGE'
```

### Performance Testing

#### Load Testing

```bash
# Install wrk with HTTP/3 support
git clone https://github.com/giltene/wrk2.git
cd wrk2
make
sudo cp wrk /usr/local/bin/wrk2

# Test QUIC/HTTP3 performance
wrk2 -t4 -c100 -d30s -R2000 --h3 https://localhost:9443/api/stats

# Test HTTP/1.1 fallback
wrk2 -t4 -c100 -d30s -R2000 http://localhost:9080/api/stats

# Gaming workload simulation
wrk2 -t8 -c200 -d60s -R5000 https://localhost:9443/api/gaming/sessions
```

#### Latency Testing

```bash
# Measure connection establishment time
curl -w "@curl-format.txt" -o /dev/null -s https://localhost:9443/api/stats

# Where curl-format.txt contains:
#     time_namelookup:  %{time_namelookup}\n
#        time_connect:  %{time_connect}\n
#     time_appconnect:  %{time_appconnect}\n
#    time_pretransfer:  %{time_pretransfer}\n
#       time_redirect:  %{time_redirect}\n
#  time_starttransfer:  %{time_starttransfer}\n
#                     ----------\n
#          time_total:  %{time_total}\n
```

## Security Considerations

### Authentication and Authorization

```toml
[security.auth]
# JWT token validation
jwt_secret = "your-jwt-secret-key"
jwt_algorithm = "HS256"
jwt_expiration = 3600  # 1 hour

# Required claims
required_claims = ["sub", "iat", "exp"]
issuer = "ghostpanel.example.com"
audience = "ghostpanel-api"

# Rate limiting per user
rate_limit_per_user = 1000  # requests per hour
rate_limit_burst = 100
```

### Network Security

```toml
[security.network]
# IP allowlist
allowed_ips = ["192.168.1.0/24", "10.0.0.0/8"]
blocked_ips = ["192.168.1.100"]  # Known malicious IPs

# DDoS protection
enable_ddos_protection = true
max_connections_per_ip = 100
connection_rate_limit = 50  # per minute

# TLS security
min_tls_version = "1.3"
cipher_suites = ["TLS_AES_256_GCM_SHA384", "TLS_CHACHA20_POLY1305_SHA256"]
```

### Request Validation

```toml
[security.validation]
# Request size limits
max_request_size = 10485760  # 10MB
max_header_size = 8192  # 8KB
max_uri_length = 2048

# Content validation
validate_content_type = true
allowed_content_types = ["application/json", "application/x-www-form-urlencoded"]

# Security headers
enable_security_headers = true
content_security_policy = "default-src 'self'"
x_frame_options = "DENY"
x_content_type_options = "nosniff"
```

## Additional Resources

- [QUIC Protocol Specification](https://tools.ietf.org/html/rfc9000)
- [HTTP/3 Specification](https://tools.ietf.org/html/rfc9114)
- [GhostPanel API Documentation](./API.md)
- [Edge Agents Documentation](./EDGE_AGENTS.md)
- [Authentication Setup](./SSO.md)
- [Bolt Container Runtime](https://bolt-containers.dev)
- [Performance Benchmarks](https://github.com/ghostpanel/benchmarks)
- [Community Discord](https://discord.gg/ghostpanel)