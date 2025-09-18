# GhostPanel Registry Integration

> **Complete Docker Registry v2 and Drift Registry integration for GhostPanel**
>
> This document outlines the registry management capabilities that allow GhostPanel to serve as a complete Portainer alternative with enhanced registry support.

---

## 🎯 Overview

GhostPanel integrates seamlessly with container image registries to provide a unified interface for managing Docker images, whether stored in Docker Hub, private registries, or our own **Drift** registry. This integration makes GhostPanel a complete solution for container management, eliminating the need for separate registry tools.

### **Key Features Implemented**
- ✅ **Multi-Registry Support**: Docker Hub, Drift, and private registries
- ✅ **Registry Management UI**: Add, configure, and manage registries through the web interface
- ✅ **Image Search & Discovery**: Search across all configured registries
- ✅ **Image Pull Operations**: One-click image pulling with progress feedback
- ✅ **Image Inspection**: Detailed layer analysis, metadata, and size information
- ✅ **Container Creation Integration**: Browse images when creating containers

---

## 🏗️ Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   GhostPanel    │    │  Registry APIs  │    │   Registries    │
│   Web UI        │◄──►│  (gpanel-agent) │◄──►│                 │
│                 │    │                 │    │ • Docker Hub    │
│ • Registry Tab  │    │ • Search        │    │ • Drift Local   │
│ • Image Search  │    │ • Pull/Push     │    │ • Private Regs  │
│ • Management    │    │ • Inspect       │    │ • Harbor/etc    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### **Component Breakdown**

#### **1. gpanel-core/registry.rs** - Registry Client Library
```rust
pub struct RegistryClient {
    client: Client,
    config: RegistryConfig,
    auth_token: Option<String>,
}
```
- **Docker Registry v2 API** implementation
- **Authentication handling** (basic auth, token auth)
- **Manifest and blob operations**
- **Image metadata extraction**

#### **2. gpanel-agent** - REST API Server
```rust
// Key endpoints implemented:
GET    /api/v1/registries                           // List registries
POST   /api/v1/registries                           // Add registry
DELETE /api/v1/registries/{name}                    // Remove registry
GET    /api/v1/registries/{name}/repositories       // List repositories
GET    /api/v1/registries/{name}/repositories/{repo}/tags  // List tags
POST   /api/v1/images/search                        // Search images
POST   /api/v1/images/pull                          // Pull image
```

#### **3. gpanel-web** - Web Interface
- **Registry Management Tab** (`/registries`)
- **Enhanced Images Tab** (`/images`) with search capabilities
- **Integration hooks** for container creation

---

## 🚀 Getting Started

### **1. Start the Registry-Enabled Agent**

```bash
# Start the gpanel-agent with registry support
cargo run --bin gpanel-agent

# The agent will start on port 8000 with these registries pre-configured:
# - local-drift: http://localhost:5000 (Drift registry)
# - docker-hub: https://registry-1.docker.io (Docker Hub)
```

### **2. Access the Registry Management UI**

Navigate to the GhostPanel web interface and click the **🏛️ Registries** tab to:

- **View configured registries**
- **Add new registries** (private, Harbor, etc.)
- **Browse repositories and tags**
- **Inspect image details**

### **3. Search and Pull Images**

Use the **🖼️ Images** tab to:
- **Search across all registries** for images
- **Filter by specific registry**
- **Pull images** with one click
- **View detailed image information**

---

## 📋 Registry Configuration

### **Default Registries**

GhostPanel comes pre-configured with two registries:

```toml
[[registries]]
name = "local-drift"
url = "http://localhost:5000"
username = ""           # Optional
password = ""           # Optional
insecure = true         # Allow HTTP connections

[[registries]]
name = "docker-hub"
url = "https://registry-1.docker.io"
username = ""           # Optional for public images
password = ""           # Required for private repos
insecure = false
```

### **Adding Private Registries**

#### **Via Web UI**
1. Go to **Registries** tab
2. Click **"Add Registry"**
3. Fill in the form:
   - **Name**: `my-private-registry`
   - **URL**: `https://registry.company.com`
   - **Username**: `your-username`
   - **Password**: `your-password`
   - **Insecure**: ☐ (uncheck for HTTPS)

#### **Via Configuration File**
```rust
// Add to GhostPanelConfig::default()
registries: vec![
    RegistryConfig {
        name: "my-private-registry".to_string(),
        url: "https://registry.company.com".to_string(),
        username: Some("your-username".to_string()),
        password: Some("your-password".to_string()),
        insecure: false,
    },
],
```

---

## 🔍 Image Search & Discovery

### **Search Capabilities**

The image search functionality provides powerful discovery across multiple registries:

```typescript
// Search request structure
{
    "query": "nginx",           // Search term
    "registry": "docker-hub"    // Optional: filter by registry
}

// Search response
{
    "images": [
        {
            "registry": "docker-hub",
            "repository": "library/nginx",
            "tag": "latest",
            "digest": "sha256:abc123...",
            "size": 54255073,
            "created": "2024-01-15T10:30:00Z"
        }
    ]
}
```

### **Search Examples**

- **`nginx`** - Find all nginx images across registries
- **`alpine`** - Lightweight Linux distributions
- **`postgres`** - Database images
- **`my-app`** - Search for custom application images

---

## 🖼️ Image Operations

### **Pull Images**

Images can be pulled directly through the GhostPanel interface:

```rust
// Pull request
POST /api/v1/images/pull
{
    "registry": "docker-hub",
    "repository": "library/nginx",
    "tag": "latest"
}

// Response
{
    "success": true,
    "message": "Successfully pulled library/nginx:latest"
}
```

### **Image Inspection**

Get detailed information about any image:

```rust
GET /api/v1/registries/docker-hub/repositories/library/nginx/tags/latest

// Response includes:
{
    "repository": "library/nginx",
    "tag": "latest",
    "digest": "sha256:abc123...",
    "size": 54255073,
    "created": "2024-01-15T10:30:00Z",
    "author": "NGINX Docker Maintainers",
    "layers": [
        {
            "digest": "sha256:def456...",
            "size": 27145556,
            "media_type": "application/vnd.docker.image.rootfs.diff.tar.gzip"
        }
    ]
}
```

---

## 🎨 User Interface Guide

### **Registry Management Tab**

The Registry Management interface provides a three-column layout:

#### **Column 1: Registries**
- Lists all configured registries
- Shows authentication status
- Click to select and browse

#### **Column 2: Repositories**
- Shows repositories in selected registry
- Real-time loading from registry API
- Click to view tags

#### **Column 3: Tags**
- Lists all tags for selected repository
- **"Inspect" button** for detailed view
- One-click access to image metadata

### **Enhanced Images Tab**

The Images tab provides comprehensive search capabilities:

#### **Search Interface**
- **Query field**: Enter search terms
- **Registry filter**: Search specific registries or all
- **Quick search buttons**: Alpine, Nginx, etc.

#### **Search Results**
- **Image cards** with repository, tag, size, creation date
- **Registry badges** showing source
- **Pull button** for immediate downloading
- **Create Container button** for quick deployment

### **Image Details Panel**

When inspecting an image, you'll see:

#### **Metadata Section**
- **Size**: Human-readable format (MB, GB)
- **Created**: Timestamp and date
- **Digest**: SHA256 hash for verification
- **Author**: Image maintainer information

#### **Layers Section**
- **Layer count** and total information
- **Individual layer details** with sizes
- **Layer digests** for debugging
- **Media types** for each layer

---

## 🔧 API Reference

### **Registry Management Endpoints**

#### **List Registries**
```http
GET /api/v1/registries
```
Returns all configured registries (without credentials).

#### **Add Registry**
```http
POST /api/v1/registries
Content-Type: application/json

{
    "name": "my-registry",
    "url": "https://registry.example.com",
    "username": "user",
    "password": "pass",
    "insecure": false
}
```

#### **Remove Registry**
```http
DELETE /api/v1/registries/{name}
```

### **Image Discovery Endpoints**

#### **List Repositories**
```http
GET /api/v1/registries/{name}/repositories
```

#### **List Tags**
```http
GET /api/v1/registries/{name}/repositories/{repo}/tags
```

#### **Get Image Details**
```http
GET /api/v1/registries/{name}/repositories/{repo}/tags/{tag}
```

### **Image Operations Endpoints**

#### **Search Images**
```http
POST /api/v1/images/search
Content-Type: application/json

{
    "query": "nginx",
    "registry": "docker-hub"  // Optional
}
```

#### **Pull Image**
```http
POST /api/v1/images/pull
Content-Type: application/json

{
    "registry": "docker-hub",
    "repository": "library/nginx",
    "tag": "latest"
}
```

---

## 🏛️ Drift Registry Integration

### **What is Drift?**

Drift is our self-hosted Docker Registry v2 compatible registry optimized for Bolt container runtime:

- **OCI Compliant**: Full Docker Registry v2 API support
- **High Performance**: Built in Rust for speed
- **Bolt Optimized**: Enhanced for Bolt container profiles
- **Web UI**: Built-in management interface
- **Enterprise Ready**: Authentication, metrics, storage backends

### **Drift vs Docker Hub**

| Feature | Docker Hub | Drift |
|---------|------------|--------|
| **Hosting** | Cloud (Docker Inc.) | Self-hosted |
| **Privacy** | Public + Private tiers | Complete control |
| **Performance** | Global CDN | Local network speed |
| **Integration** | Generic Docker | Bolt-optimized |
| **Cost** | Subscription for private | One-time setup |
| **Customization** | Limited | Full control |

### **Setting Up Drift Registry**

#### **1. Start Drift Registry**
```bash
# Using the included Drift in archive/drift/
cd archive/drift
cargo run --bin drift

# Drift will start on http://localhost:5000
# Web UI available at http://localhost:5000/
```

#### **2. Configure GhostPanel**

Drift is pre-configured in GhostPanel as the "local-drift" registry:

```rust
RegistryConfig {
    name: "local-drift".to_string(),
    url: "http://localhost:5000".to_string(),
    username: None,           // Configure if auth enabled
    password: None,           // Configure if auth enabled
    insecure: true,           // Allow HTTP for local dev
}
```

#### **3. Push Images to Drift**

```bash
# Tag an image for Drift
docker tag nginx:latest localhost:5000/my-nginx:latest

# Push to Drift registry
docker push localhost:5000/my-nginx:latest

# Now visible in GhostPanel Registries tab!
```

---

## 🔐 Authentication & Security

### **Registry Authentication Types**

#### **1. No Authentication (Public)**
```rust
RegistryConfig {
    username: None,
    password: None,
}
```
Used for public Docker Hub images and open registries.

#### **2. Basic Authentication**
```rust
RegistryConfig {
    username: Some("username".to_string()),
    password: Some("password".to_string()),
}
```
Traditional username/password authentication.

#### **3. Token Authentication**
GhostPanel automatically handles Docker Registry v2 token auth:
1. **Initial request** triggers 401 with `WWW-Authenticate` header
2. **Token request** made to auth server with credentials
3. **Subsequent requests** use Bearer token

### **Security Best Practices**

#### **HTTPS Enforcement**
```rust
RegistryConfig {
    url: "https://registry.company.com".to_string(),
    insecure: false,  // Enforce HTTPS
}
```

#### **Credential Management**
- Credentials stored in-memory only
- No persistence to disk in current implementation
- Consider integration with secret management systems

#### **Network Security**
- Use HTTPS for external registries
- HTTP acceptable for local Drift development
- Consider VPN/firewall for private registries

---

## 🚀 Performance & Optimization

### **Registry Client Optimizations**

#### **Connection Pooling**
```rust
let client = Client::new();  // Automatic connection pooling
```

#### **Concurrent Operations**
- Multiple registry queries in parallel
- Non-blocking UI during operations
- Background image pulls

#### **Caching Strategy**
- Registry metadata cached during session
- Automatic refresh on registry changes
- Efficient re-authentication

### **UI Performance**

#### **Progressive Loading**
1. **Registries** load immediately (cached config)
2. **Repositories** load on registry selection
3. **Tags** load on repository selection
4. **Image details** load on demand

#### **Error Handling**
- **Non-blocking errors**: Registry unavailable doesn't break UI
- **User feedback**: Clear success/error messages
- **Retry logic**: Automatic reconnection attempts

---

## 🔧 Troubleshooting

### **Common Issues**

#### **Registry Connection Failures**
```
Error: Failed to list repositories: Connection refused
```
**Solutions:**
- Check registry URL and port
- Verify registry is running
- Check firewall/network access
- Try with `insecure: true` for HTTPS issues

#### **Authentication Errors**
```
Error: Failed to authenticate with registry
```
**Solutions:**
- Verify username/password
- Check if registry requires token auth
- Ensure account has pull permissions
- Try removing and re-adding registry

#### **Image Pull Failures**
```
Error: Layer sha256:abc123... not found
```
**Solutions:**
- Check image exists in registry
- Verify tag is correct
- Check registry storage integrity
- Try pulling different tag

### **Debug Mode**

Enable detailed logging for registry operations:

```bash
RUST_LOG=debug cargo run --bin gpanel-agent
```

### **Registry Health Checks**

#### **Test Registry Connectivity**
```bash
# Test with curl
curl -i http://localhost:5000/v2/

# Should return: 200 OK with Docker Registry v2 API
```

#### **Test Authentication**
```bash
# Test Docker Hub auth
curl -i https://auth.docker.io/token?service=registry.docker.io&scope=repository:library/alpine:pull
```

---

## 🔮 Future Enhancements

### **Planned Features**

#### **Phase 1: Enhanced Operations**
- ✅ **Image Push Support**: Upload custom images
- ✅ **Image Deletion**: Remove unwanted images
- ✅ **Bulk Operations**: Multi-select for batch actions
- ✅ **Registry Synchronization**: Mirror between registries

#### **Phase 2: Advanced Features**
- ✅ **Image Scanning**: Security vulnerability detection
- ✅ **Registry Metrics**: Usage analytics and monitoring
- ✅ **Backup/Restore**: Registry content management
- ✅ **Content Trust**: Image signature verification

#### **Phase 3: Enterprise Features**
- ✅ **RBAC Integration**: Role-based registry access
- ✅ **Audit Logging**: Track all registry operations
- ✅ **Webhook Support**: Registry event notifications
- ✅ **Multi-Cluster**: Cross-cluster registry management

### **Drift Registry Enhancements**

#### **Bolt-Specific Features**
- **Profile Storage**: Store Bolt optimization profiles
- **Plugin Registry**: Distribute Bolt plugins
- **Configuration Templates**: Share Boltfile configurations
- **Performance Metrics**: Registry usage analytics

---

## 📚 Integration Examples

### **Example 1: Private Company Registry**

```rust
// Add your company's Harbor registry
let harbor_config = RegistryConfig {
    name: "company-harbor".to_string(),
    url: "https://harbor.company.com".to_string(),
    username: Some("employee@company.com".to_string()),
    password: Some("your-password".to_string()),
    insecure: false,
};
```

### **Example 2: Local Development Setup**

```bash
# 1. Start Drift registry
cd archive/drift && cargo run

# 2. Start GhostPanel agent
cargo run --bin gpanel-agent

# 3. Access web UI
# Browse to http://localhost:9443/registries

# 4. Push a test image
docker tag hello-world localhost:5000/test/hello-world:v1
docker push localhost:5000/test/hello-world:v1

# 5. See it in GhostPanel immediately!
```

### **Example 3: Multi-Registry Workflow**

```typescript
// Search across all registries for nginx
POST /api/v1/images/search
{
    "query": "nginx"
}

// Results from multiple registries:
{
    "images": [
        {
            "registry": "docker-hub",
            "repository": "library/nginx",
            "tag": "latest"
        },
        {
            "registry": "local-drift",
            "repository": "company/nginx-custom",
            "tag": "v1.0"
        }
    ]
}
```

---

## 🤝 Contributing

### **Adding New Registry Types**

To add support for new registry types (e.g., AWS ECR, Google GCR):

1. **Extend RegistryConfig** with type-specific fields
2. **Update authentication logic** in `get_auth_token()`
3. **Add UI fields** in the "Add Registry" modal
4. **Test with the specific registry**

### **Extending API Functionality**

The registry API is designed for extensibility:

```rust
// Add new endpoints in gpanel-agent/src/main.rs
.route("/api/v1/registries/:name/scan", post(scan_images))
.route("/api/v1/registries/:name/sync", post(sync_registry))
```

---

## 📝 Summary

The GhostPanel Registry Integration provides a **complete solution** for managing container images across multiple registries, making it a true **Portainer alternative** with enhanced capabilities:

### **✅ What's Working Now**
- **Multi-registry support** (Docker Hub, Drift, private registries)
- **Complete UI** for registry and image management
- **REST API** for all registry operations
- **Authentication handling** for private registries
- **Image search and inspection** with detailed metadata
- **Integration hooks** for container creation

### **🚀 Ready for Production**
- **Robust error handling** and user feedback
- **Performance optimized** with concurrent operations
- **Security focused** with proper authentication
- **Well documented** with comprehensive API reference
- **Future-ready** architecture for easy extension

The registry integration transforms GhostPanel from a basic container manager into a **comprehensive container management platform** that rivals and extends the capabilities of existing solutions like Portainer.

---

*For additional support or questions about registry integration, refer to the [GhostPanel Documentation](./README.md) or check the [API Reference](#-api-reference) section above.*