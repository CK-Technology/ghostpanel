# GhostPanel API Documentation

GhostPanel provides a comprehensive REST API for managing Bolt containers, images, networks, volumes, and gaming environments. The API is designed to be similar to Docker's API but optimized for Bolt containers and gaming workloads.

## Base URL

```
Production: https://your-domain.com:9443/api
Development: https://localhost:9443/api
```

## Authentication

All API requests require authentication via JWT tokens obtained through OAuth2 flow.

### JWT Token

Include the JWT token in the `Authorization` header:

```http
Authorization: Bearer <jwt_token>
```

### OAuth2 Providers

Supported OAuth2 providers:
- **Azure AD**: `/auth/azure`
- **Google**: `/auth/google`
- **GitHub**: `/auth/github`

## API Endpoints Overview

| Resource | Base Path | Description |
|----------|-----------|-------------|
| Authentication | `/auth` | OAuth2 authentication flows |
| Containers | `/containers` | Container lifecycle management |
| Images | `/images` | Image operations and registry |
| Networks | `/networks` | Network management |
| Volumes | `/volumes` | Volume operations |
| Gaming | `/gaming` | Gaming-specific operations |
| System | `/system` | System information and stats |
| Stats | `/stats` | Proxy statistics |

## Authentication Endpoints

### Start OAuth2 Flow

```http
GET /auth/{provider}
```

**Parameters:**
- `provider`: `azure`, `google`, or `github`

**Response:**
```http
HTTP/1.1 302 Found
Location: https://oauth-provider.com/authorize?client_id=...
```

### OAuth2 Callback

```http
GET /auth/{provider}/callback?code={code}&state={state}
```

**Response:**
```json
{
  "access_token": "jwt_token_here",
  "token_type": "Bearer",
  "expires_in": 3600,
  "user": {
    "id": "user_id",
    "email": "user@example.com",
    "name": "User Name",
    "avatar_url": "https://avatar-url.com"
  }
}
```

### Logout

```http
POST /auth/logout
Authorization: Bearer <jwt_token>
```

**Response:**
```http
HTTP/1.1 204 No Content
```

## Container Endpoints

### List Containers

```http
GET /containers
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `all`: Include stopped containers (default: `false`)
- `size`: Include container size information (default: `false`)
- `filters`: JSON encoded filters

**Response:**
```json
[
  {
    "id": "container_id",
    "name": "container_name",
    "image": "image:tag",
    "command": "command",
    "created": "2024-01-01T00:00:00Z",
    "status": "running",
    "ports": [
      {
        "private_port": 8080,
        "public_port": 8080,
        "type": "tcp"
      }
    ],
    "size_rw": 1024,
    "size_root_fs": 1048576
  }
]
```

### Create Container

```http
POST /containers/create
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "my-container",
  "image": "ubuntu:latest",
  "cmd": ["/bin/bash"],
  "env": ["ENV_VAR=value"],
  "exposed_ports": {
    "8080/tcp": {}
  },
  "host_config": {
    "port_bindings": {
      "8080/tcp": [
        {
          "host_ip": "0.0.0.0",
          "host_port": "8080"
        }
      ]
    },
    "binds": [
      "/host/path:/container/path:rw"
    ],
    "memory": 1073741824,
    "cpu_shares": 512,
    "restart_policy": {
      "name": "unless-stopped"
    }
  },
  "networking_config": {
    "endpoints_config": {
      "bridge": {}
    }
  }
}
```

**Response:**
```json
{
  "id": "new_container_id",
  "warnings": []
}
```

### Get Container

```http
GET /containers/{id}
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "id": "container_id",
  "created": "2024-01-01T00:00:00Z",
  "path": "command",
  "args": ["arg1", "arg2"],
  "state": {
    "status": "running",
    "running": true,
    "paused": false,
    "restarting": false,
    "pid": 1234,
    "exit_code": 0,
    "started_at": "2024-01-01T00:00:00Z",
    "finished_at": "0001-01-01T00:00:00Z"
  },
  "image": "sha256:image_id",
  "name": "/container_name",
  "restart_count": 0,
  "config": {
    "hostname": "container_id",
    "user": "",
    "memory": 0,
    "memory_swap": 0,
    "cpu_shares": 0,
    "env": ["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"],
    "cmd": ["/bin/bash"],
    "image": "ubuntu:latest",
    "working_dir": "/",
    "entrypoint": null,
    "labels": {}
  },
  "host_config": {
    "binds": [],
    "network_mode": "default",
    "port_bindings": {},
    "restart_policy": {
      "name": "no",
      "maximum_retry_count": 0
    }
  }
}
```

### Start Container

```http
POST /containers/{id}/start
Authorization: Bearer <jwt_token>
```

**Response:**
```http
HTTP/1.1 204 No Content
```

### Stop Container

```http
POST /containers/{id}/stop
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `t`: Number of seconds to wait before killing (default: 10)

**Response:**
```http
HTTP/1.1 204 No Content
```

### Restart Container

```http
POST /containers/{id}/restart
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `t`: Number of seconds to wait before killing (default: 10)

**Response:**
```http
HTTP/1.1 204 No Content
```

### Remove Container

```http
DELETE /containers/{id}
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `force`: Force removal of running container (default: `false`)
- `v`: Remove volumes associated with container (default: `false`)

**Response:**
```http
HTTP/1.1 204 No Content
```

### Container Logs

```http
GET /containers/{id}/logs
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `stdout`: Include stdout (default: `true`)
- `stderr`: Include stderr (default: `true`)
- `timestamps`: Include timestamps (default: `false`)
- `tail`: Number of lines from end of logs (default: `all`)
- `since`: Show logs since timestamp
- `until`: Show logs until timestamp
- `follow`: Stream logs (default: `false`)

**Response:**
```
Content-Type: text/plain

2024-01-01T00:00:00.000000000Z Log message 1
2024-01-01T00:00:01.000000000Z Log message 2
```

### Container Stats

```http
GET /containers/{id}/stats
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `stream`: Stream statistics (default: `false`)

**Response:**
```json
{
  "read": "2024-01-01T00:00:00.000000000Z",
  "preread": "2024-01-01T00:00:00.000000000Z",
  "pids_stats": {
    "current": 10
  },
  "blkio_stats": {
    "io_service_bytes_recursive": [],
    "io_serviced_recursive": []
  },
  "num_procs": 0,
  "storage_stats": {},
  "cpu_stats": {
    "cpu_usage": {
      "total_usage": 12345678,
      "percpu_usage": [12345678],
      "usage_in_kernelmode": 1234567,
      "usage_in_usermode": 1234567
    },
    "system_cpu_usage": 123456789,
    "online_cpus": 4,
    "throttling_data": {
      "periods": 0,
      "throttled_periods": 0,
      "throttled_time": 0
    }
  },
  "precpu_stats": {
    "cpu_usage": {
      "total_usage": 12345678
    }
  },
  "memory_stats": {
    "usage": 16777216,
    "max_usage": 33554432,
    "stats": {
      "active_anon": 8388608,
      "active_file": 4194304,
      "cache": 8388608,
      "dirty": 0,
      "hierarchical_memory_limit": 9223372036854775807,
      "hierarchical_memsw_limit": 9223372036854775807,
      "inactive_anon": 0,
      "inactive_file": 4194304,
      "mapped_file": 0,
      "pgfault": 1000,
      "pgmajfault": 10,
      "pgpgin": 500,
      "pgpgout": 400,
      "rss": 8388608,
      "rss_huge": 0,
      "total_active_anon": 8388608,
      "total_active_file": 4194304,
      "total_cache": 8388608,
      "total_dirty": 0,
      "total_inactive_anon": 0,
      "total_inactive_file": 4194304,
      "total_mapped_file": 0,
      "total_pgfault": 1000,
      "total_pgmajfault": 10,
      "total_pgpgin": 500,
      "total_pgpgout": 400,
      "total_rss": 8388608,
      "total_rss_huge": 0,
      "total_unevictable": 0,
      "unevictable": 0
    },
    "limit": 1073741824
  },
  "name": "/container_name",
  "id": "container_id",
  "networks": {
    "eth0": {
      "rx_bytes": 1024,
      "rx_packets": 10,
      "rx_errors": 0,
      "rx_dropped": 0,
      "tx_bytes": 512,
      "tx_packets": 5,
      "tx_errors": 0,
      "tx_dropped": 0
    }
  }
}
```

## Image Endpoints

### List Images

```http
GET /images
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `all`: Show all images including intermediates (default: `false`)
- `filters`: JSON encoded filters
- `digests`: Show image digests (default: `false`)

**Response:**
```json
[
  {
    "id": "sha256:image_id",
    "parent_id": "sha256:parent_id",
    "repo_tags": ["ubuntu:latest", "ubuntu:22.04"],
    "repo_digests": ["ubuntu@sha256:digest"],
    "created": 1640995200,
    "size": 72788992,
    "virtual_size": 72788992,
    "shared_size": 0,
    "labels": {},
    "containers": 2
  }
]
```

### Pull Image

```http
POST /images/create
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `fromImage`: Image name to pull
- `tag`: Tag of image to pull (default: `latest`)

**Response:**
```json
[
  {"status": "Pulling from library/ubuntu"},
  {"status": "Pulling fs layer", "id": "layer_id"},
  {"status": "Download complete", "id": "layer_id"},
  {"status": "Pull complete", "id": "layer_id"},
  {"status": "Status: Downloaded newer image for ubuntu:latest"}
]
```

### Get Image

```http
GET /images/{name}
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "id": "sha256:image_id",
  "repo_tags": ["ubuntu:latest"],
  "repo_digests": ["ubuntu@sha256:digest"],
  "parent": "sha256:parent_id",
  "comment": "",
  "created": "2024-01-01T00:00:00.000000000Z",
  "container": "container_id",
  "config": {
    "hostname": "",
    "user": "",
    "env": ["PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"],
    "cmd": ["/bin/bash"],
    "image": "sha256:image_id",
    "working_dir": "/",
    "entrypoint": null,
    "labels": {}
  },
  "architecture": "amd64",
  "os": "linux",
  "size": 72788992,
  "virtual_size": 72788992,
  "root_fs": {
    "type": "layers",
    "layers": ["sha256:layer1", "sha256:layer2"]
  }
}
```

### Remove Image

```http
DELETE /images/{name}
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `force`: Force removal (default: `false`)
- `noprune`: Don't delete untagged parents (default: `false`)

**Response:**
```json
[
  {"untagged": "ubuntu:latest"},
  {"deleted": "sha256:image_id"}
]
```

## Network Endpoints

### List Networks

```http
GET /networks
Authorization: Bearer <jwt_token>
```

**Response:**
```json
[
  {
    "name": "bridge",
    "id": "network_id",
    "created": "2024-01-01T00:00:00.000000000Z",
    "scope": "local",
    "driver": "bridge",
    "enable_ipv6": false,
    "ipam": {
      "driver": "default",
      "options": {},
      "config": [
        {
          "subnet": "172.17.0.0/16",
          "gateway": "172.17.0.1"
        }
      ]
    },
    "internal": false,
    "attachable": false,
    "ingress": false,
    "containers": {
      "container_id": {
        "name": "container_name",
        "endpoint_id": "endpoint_id",
        "mac_address": "02:42:ac:11:00:02",
        "ipv4_address": "172.17.0.2/16",
        "ipv6_address": ""
      }
    },
    "options": {
      "com.docker.network.bridge.default_bridge": "true"
    },
    "labels": {}
  }
]
```

### Create Network

```http
POST /networks/create
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "my-network",
  "driver": "bridge",
  "ipam": {
    "config": [
      {
        "subnet": "192.168.1.0/24",
        "gateway": "192.168.1.1"
      }
    ]
  },
  "options": {
    "com.docker.network.bridge.enable_icc": "true"
  },
  "labels": {
    "purpose": "gaming"
  }
}
```

**Response:**
```json
{
  "id": "network_id",
  "warning": ""
}
```

### Remove Network

```http
DELETE /networks/{id}
Authorization: Bearer <jwt_token>
```

**Response:**
```http
HTTP/1.1 204 No Content
```

## Volume Endpoints

### List Volumes

```http
GET /volumes
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `filters`: JSON encoded filters

**Response:**
```json
{
  "volumes": [
    {
      "created_at": "2024-01-01T00:00:00.000000000Z",
      "driver": "local",
      "labels": {},
      "mountpoint": "/var/lib/bolt/volumes/volume_name/_data",
      "name": "volume_name",
      "options": {},
      "scope": "local"
    }
  ],
  "warnings": []
}
```

### Create Volume

```http
POST /volumes/create
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "name": "my-volume",
  "driver": "local",
  "driver_opts": {},
  "labels": {
    "purpose": "gaming-data"
  }
}
```

**Response:**
```json
{
  "created_at": "2024-01-01T00:00:00.000000000Z",
  "driver": "local",
  "labels": {
    "purpose": "gaming-data"
  },
  "mountpoint": "/var/lib/bolt/volumes/my-volume/_data",
  "name": "my-volume",
  "options": {},
  "scope": "local"
}
```

### Get Volume

```http
GET /volumes/{name}
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "created_at": "2024-01-01T00:00:00.000000000Z",
  "driver": "local",
  "labels": {
    "purpose": "gaming-data"
  },
  "mountpoint": "/var/lib/bolt/volumes/my-volume/_data",
  "name": "my-volume",
  "options": {},
  "scope": "local"
}
```

### Remove Volume

```http
DELETE /volumes/{name}
Authorization: Bearer <jwt_token>
```

**Query Parameters:**
- `force`: Force removal (default: `false`)

**Response:**
```http
HTTP/1.1 204 No Content
```

## Gaming Endpoints

### List Gaming Sessions

```http
GET /gaming/sessions
Authorization: Bearer <jwt_token>
```

**Response:**
```json
[
  {
    "id": "session_id",
    "container_id": "container_id",
    "game": "steam",
    "status": "running",
    "created": "2024-01-01T00:00:00Z",
    "gpu_allocated": true,
    "gpu_device": "/dev/dri/card0",
    "proton_version": "8.0-5",
    "steam_app_id": "12345",
    "display": ":1",
    "vnc_port": 5901,
    "audio_enabled": true,
    "performance": {
      "fps": 60,
      "cpu_usage": 45.2,
      "gpu_usage": 78.5,
      "memory_usage": 65.3,
      "temperature": 72
    }
  }
]
```

### Create Gaming Session

```http
POST /gaming/sessions
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "game": "steam",
  "steam_app_id": "12345",
  "proton_version": "8.0-5",
  "gpu_required": true,
  "audio_enabled": true,
  "resolution": "1920x1080",
  "config": {
    "env": {
      "STEAM_COMPAT_DATA_PATH": "/home/steam/.steam/steam/steamapps/compatdata"
    },
    "mounts": [
      "/host/games:/games:rw"
    ]
  }
}
```

**Response:**
```json
{
  "id": "session_id",
  "container_id": "container_id",
  "vnc_url": "vnc://localhost:5901",
  "web_url": "https://localhost:9443/gaming/sessions/session_id"
}
```

### Get Gaming Session

```http
GET /gaming/sessions/{id}
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "id": "session_id",
  "container_id": "container_id",
  "game": "steam",
  "status": "running",
  "created": "2024-01-01T00:00:00Z",
  "config": {
    "steam_app_id": "12345",
    "proton_version": "8.0-5",
    "resolution": "1920x1080"
  },
  "performance": {
    "fps": 60,
    "cpu_usage": 45.2,
    "gpu_usage": 78.5,
    "memory_usage": 65.3,
    "temperature": 72
  },
  "urls": {
    "vnc": "vnc://localhost:5901",
    "web": "https://localhost:9443/gaming/sessions/session_id"
  }
}
```

### Stop Gaming Session

```http
DELETE /gaming/sessions/{id}
Authorization: Bearer <jwt_token>
```

**Response:**
```http
HTTP/1.1 204 No Content
```

### Gaming Templates

```http
GET /gaming/templates
Authorization: Bearer <jwt_token>
```

**Response:**
```json
[
  {
    "id": "steam-template",
    "name": "Steam Gaming",
    "description": "Steam client with Proton support",
    "image": "ghostpanel/steam:latest",
    "gpu_required": true,
    "proton_versions": ["8.0-5", "7.0-6", "6.3-8"],
    "default_proton": "8.0-5",
    "supported_games": ["Counter-Strike 2", "Dota 2", "Half-Life: Alyx"]
  }
]
```

## System Endpoints

### System Information

```http
GET /system/info
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "id": "system_id",
  "containers": 5,
  "containers_running": 3,
  "containers_paused": 0,
  "containers_stopped": 2,
  "images": 10,
  "driver": "bolt",
  "driver_status": [
    ["Bolt Version", "2.0.0"],
    ["Storage Driver", "overlay2"]
  ],
  "system_status": null,
  "plugins": {
    "volume": ["local"],
    "network": ["bridge", "host", "null"],
    "authorization": null,
    "log": ["json-file", "journald"]
  },
  "memory_limit": true,
  "swap_limit": true,
  "kernel_memory": true,
  "cpu_cfs_period": true,
  "cpu_cfs_quota": true,
  "cpu_shares": true,
  "cpu_set": true,
  "pids_limit": true,
  "ipv4_forwarding": true,
  "bridge_nf_iptables": true,
  "bridge_nf_ip6tables": true,
  "debug": false,
  "nfd": 42,
  "oom_kill_disable": true,
  "n_goroutines": 135,
  "system_time": "2024-01-01T00:00:00.000000000Z",
  "logging_driver": "json-file",
  "cgroup_driver": "systemd",
  "n_events_listener": 5,
  "kernel_version": "6.5.0-1-default",
  "operating_system": "Ubuntu 22.04.3 LTS",
  "os_type": "linux",
  "architecture": "x86_64",
  "ncpu": 8,
  "mem_total": 16777216000,
  "bolt_root_dir": "/var/lib/bolt",
  "http_proxy": "",
  "https_proxy": "",
  "no_proxy": "",
  "name": "ghostpanel-host",
  "labels": ["gaming-optimized", "gpu-enabled"],
  "experimental_build": false,
  "server_version": "2.0.0",
  "cluster_store": "",
  "cluster_advertise": "",
  "runtimes": {
    "runc": {
      "path": "runc"
    }
  },
  "default_runtime": "runc",
  "swarm": {
    "node_id": "",
    "node_addr": "",
    "local_node_state": "inactive",
    "control_available": false,
    "error": "",
    "remote_managers": null
  },
  "live_restore_enabled": false,
  "isolation": "",
  "init_binary": "bolt-init",
  "containerd_commit": {
    "id": "commit_id",
    "expected": "commit_id"
  },
  "runc_commit": {
    "id": "commit_id",
    "expected": "commit_id"
  },
  "init_commit": {
    "id": "commit_id",
    "expected": "commit_id"
  },
  "security_options": ["name=seccomp,profile=default"]
}
```

### System Stats

```http
GET /system/stats
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "cpu": {
    "usage_percent": 25.5,
    "load_average": [1.2, 1.5, 1.8],
    "cores": 8
  },
  "memory": {
    "total": 16777216000,
    "used": 8388608000,
    "available": 8388608000,
    "usage_percent": 50.0
  },
  "disk": {
    "total": 1000000000000,
    "used": 500000000000,
    "available": 500000000000,
    "usage_percent": 50.0
  },
  "network": {
    "bytes_sent": 1048576000,
    "bytes_recv": 2097152000,
    "packets_sent": 1000000,
    "packets_recv": 2000000
  },
  "gpu": [
    {
      "id": 0,
      "name": "NVIDIA GeForce RTX 4080",
      "memory_total": 16777216000,
      "memory_used": 4194304000,
      "usage_percent": 75.0,
      "temperature": 72
    }
  ]
}
```

## Proxy Stats Endpoint

### Get Proxy Statistics

```http
GET /stats
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "active_connections": 42,
  "total_requests": 1000000,
  "quic_requests": 750000,
  "http_requests": 250000,
  "bytes_transferred": 1073741824000,
  "uptime_seconds": 86400
}
```

## Error Responses

All endpoints may return these error responses:

### Authentication Errors

```http
HTTP/1.1 401 Unauthorized
Content-Type: application/json

{
  "error": "Unauthorized",
  "message": "Invalid or missing JWT token"
}
```

### Authorization Errors

```http
HTTP/1.1 403 Forbidden
Content-Type: application/json

{
  "error": "Forbidden",
  "message": "Insufficient permissions"
}
```

### Not Found Errors

```http
HTTP/1.1 404 Not Found
Content-Type: application/json

{
  "error": "Not Found",
  "message": "Container not found"
}
```

### Validation Errors

```http
HTTP/1.1 400 Bad Request
Content-Type: application/json

{
  "error": "Bad Request",
  "message": "Invalid request body",
  "details": [
    {
      "field": "name",
      "message": "Container name is required"
    }
  ]
}
```

### Server Errors

```http
HTTP/1.1 500 Internal Server Error
Content-Type: application/json

{
  "error": "Internal Server Error",
  "message": "Failed to communicate with Bolt daemon"
}
```

## Rate Limiting

API requests are rate limited to prevent abuse:

- **Authenticated users**: 1000 requests per hour
- **Per endpoint limits**: Some endpoints have lower limits
- **WebSocket connections**: 10 concurrent connections per user

Rate limit headers are included in responses:

```http
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995200
```

## WebSocket Endpoints

### Container Logs (Streaming)

```
ws://localhost:9443/api/containers/{id}/logs/ws
Authorization: Bearer <jwt_token>
```

### Container Stats (Streaming)

```
ws://localhost:9443/api/containers/{id}/stats/ws
Authorization: Bearer <jwt_token>
```

### System Events

```
ws://localhost:9443/api/events/ws
Authorization: Bearer <jwt_token>
```

### Gaming Session Stream

```
ws://localhost:9443/api/gaming/sessions/{id}/stream
Authorization: Bearer <jwt_token>
```

## SDK Examples

### Rust SDK

```rust
use ghostpanel_sdk::GhostPanelClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GhostPanelClient::new("https://localhost:9443")
        .with_token("jwt_token_here");

    // List containers
    let containers = client.containers().list().await?;
    println!("Containers: {:#?}", containers);

    // Create container
    let container = client.containers()
        .create("ubuntu:latest")
        .name("my-container")
        .port_binding("8080", "8080")
        .send()
        .await?;

    // Start container
    client.containers().start(&container.id).await?;

    Ok(())
}
```

### JavaScript SDK

```javascript
import { GhostPanelClient } from '@ghostpanel/js-sdk';

const client = new GhostPanelClient({
  baseUrl: 'https://localhost:9443',
  token: 'jwt_token_here'
});

// List containers
const containers = await client.containers.list();
console.log('Containers:', containers);

// Create and start container
const container = await client.containers.create({
  image: 'ubuntu:latest',
  name: 'my-container',
  portBindings: { '8080': '8080' }
});

await client.containers.start(container.id);
```

### Python SDK

```python
import asyncio
from ghostpanel import GhostPanelClient

async def main():
    client = GhostPanelClient(
        base_url="https://localhost:9443",
        token="jwt_token_here"
    )

    # List containers
    containers = await client.containers.list()
    print("Containers:", containers)

    # Create container
    container = await client.containers.create(
        image="ubuntu:latest",
        name="my-container",
        port_bindings={"8080": "8080"}
    )

    # Start container
    await client.containers.start(container.id)

asyncio.run(main())
```

## Additional Resources

- [Authentication Setup](./SSO.md)
- [Azure AD Configuration](./AZURE.md)
- [Google OAuth2 Configuration](./GOOGLE.md)
- [GitHub OAuth Configuration](./GITHUB.md)
- [Edge Agents Documentation](./EDGE_AGENTS.md)
- [Socket Proxy Documentation](./SOCKET_PROXY.md)
- [Bolt Container Documentation](https://bolt-containers.dev/docs)