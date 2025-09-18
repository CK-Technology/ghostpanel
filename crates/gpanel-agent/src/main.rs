use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use gpanel_core::{
    GhostPanelConfig, RegistryConfig, RegistryManager,
    ImageInfo, RepositoryList, TagList,
    BoltClient, MockBoltClient, Container, CreateContainerRequest, ContainerFilter,
    ContainerLogsRequest, ContainerStats
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: GhostPanelConfig,
    pub registry_manager: Arc<RwLock<RegistryManager>>,
    pub bolt_client: Arc<MockBoltClient>, // Use MockBoltClient for now
}

/// Registry list response for API
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryListResponse {
    pub registries: Vec<RegistryConfigResponse>,
}

/// Registry configuration response (without credentials)
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistryConfigResponse {
    pub name: String,
    pub url: String,
    pub has_auth: bool,
    pub insecure: bool,
}

/// Add registry request
#[derive(Debug, Serialize, Deserialize)]
pub struct AddRegistryRequest {
    pub name: String,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub insecure: bool,
}

/// Image search request
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchRequest {
    pub query: String,
    pub registry: Option<String>,
}

/// Query parameters for GET image search
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchQuery {
    pub q: String,
    pub registry: Option<String>,
}

/// Image search response
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchResponse {
    pub images: Vec<ImageSearchResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageSearchResult {
    pub registry: String,
    pub repository: String,
    pub tag: String,
    pub digest: String,
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
}

/// Image pull request
#[derive(Debug, Serialize, Deserialize)]
pub struct ImagePullRequest {
    pub registry: String,
    pub repository: String,
    pub tag: String,
}

/// Operation result response
#[derive(Debug, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}

/// Container list response
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerListResponse {
    pub containers: Vec<Container>,
}

/// Container operation request
#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerOperationRequest {
    pub action: String,
    pub timeout: Option<u32>,
    pub force: Option<bool>,
    pub remove_volumes: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    info!("Starting GhostPanel Agent...");

    // Load configuration
    let config = GhostPanelConfig::default();

    // Initialize registry manager with default registries
    let mut registry_manager = RegistryManager::new();

    // Add configured registries
    for registry_config in &config.registries {
        match registry_manager.add_registry(registry_config.clone()).await {
            Ok(_) => info!("Added registry: {}", registry_config.name),
            Err(e) => error!("Failed to add registry {}: {}", registry_config.name, e),
        }
    }

    // Initialize Bolt client (using mock for now)
    let bolt_client = Arc::new(MockBoltClient::new());

    let state = AppState {
        config: config.clone(),
        registry_manager: Arc::new(RwLock::new(registry_manager)),
        bolt_client,
    };

    // Build the router
    let app = Router::new()
        // Container management endpoints
        .route("/api/v1/containers", get(list_containers))
        .route("/api/v1/containers", post(create_container))
        .route("/api/v1/containers/:id", get(get_container))
        .route("/api/v1/containers/:id", delete(delete_container))
        .route("/api/v1/containers/:id/start", post(start_container))
        .route("/api/v1/containers/:id/stop", post(stop_container))
        .route("/api/v1/containers/:id/restart", post(restart_container))
        .route("/api/v1/containers/:id/logs", get(get_container_logs))
        .route("/api/v1/containers/:id/stats", get(get_container_stats))

        // Registry management endpoints
        .route("/api/v1/registries", get(list_registries))
        .route("/api/v1/registries", post(add_registry))
        .route("/api/v1/registries/:name", delete(remove_registry))

        // Image operations
        .route("/api/v1/registries/:name/repositories", get(list_repositories))
        .route("/api/v1/registries/:name/repositories/:repo/tags", get(list_tags))
        .route("/api/v1/registries/:name/repositories/:repo/tags/:tag", get(get_image_info))

        // Image management
        .route("/api/v1/images/search", get(search_images_get))
        .route("/api/v1/images/search", post(search_images))
        .route("/api/v1/images/pull", post(pull_image))

        // Health check
        .route("/health", get(health_check))
        .route("/api/v1/health", get(health_check))

        // Add state and middleware
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .into_inner()
        );

    // Start the server
    let bind_addr = format!("0.0.0.0:{}", config.agent_port);
    info!("GhostPanel Agent listening on {}", bind_addr);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "gpanel-agent",
        "timestamp": chrono::Utc::now()
    }))
}

/// List all configured registries
async fn list_registries(State(state): State<AppState>) -> Result<Json<RegistryListResponse>, StatusCode> {
    let registries: Vec<RegistryConfigResponse> = state.config.registries
        .iter()
        .map(|r| RegistryConfigResponse {
            name: r.name.clone(),
            url: r.url.clone(),
            has_auth: r.username.is_some() && r.password.is_some(),
            insecure: r.insecure,
        })
        .collect();

    Ok(Json(RegistryListResponse { registries }))
}

/// Add a new registry
async fn add_registry(
    State(state): State<AppState>,
    Json(request): Json<AddRegistryRequest>,
) -> Result<Json<OperationResult>, StatusCode> {
    let registry_config = RegistryConfig {
        name: request.name.clone(),
        url: request.url,
        username: request.username,
        password: request.password,
        insecure: request.insecure,
    };

    let mut manager = state.registry_manager.write().await;

    match manager.add_registry(registry_config).await {
        Ok(_) => {
            info!("Successfully added registry: {}", request.name);
            Ok(Json(OperationResult {
                success: true,
                message: format!("Registry '{}' added successfully", request.name),
            }))
        }
        Err(e) => {
            error!("Failed to add registry {}: {}", request.name, e);
            Ok(Json(OperationResult {
                success: false,
                message: format!("Failed to add registry: {}", e),
            }))
        }
    }
}

/// Remove a registry
async fn remove_registry(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<OperationResult>, StatusCode> {
    let mut manager = state.registry_manager.write().await;

    if manager.remove_registry(&name) {
        info!("Successfully removed registry: {}", name);
        Ok(Json(OperationResult {
            success: true,
            message: format!("Registry '{}' removed successfully", name),
        }))
    } else {
        Ok(Json(OperationResult {
            success: false,
            message: format!("Registry '{}' not found", name),
        }))
    }
}

/// List repositories in a specific registry
async fn list_repositories(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<RepositoryList>, StatusCode> {
    let manager = state.registry_manager.read().await;

    if let Some(client) = manager.get_registry(&name) {
        match client.list_repositories().await {
            Ok(repositories) => Ok(Json(RepositoryList { repositories })),
            Err(e) => {
                error!("Failed to list repositories for {}: {}", name, e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        error!("Registry not found: {}", name);
        Err(StatusCode::NOT_FOUND)
    }
}

/// List tags for a repository
async fn list_tags(
    State(state): State<AppState>,
    Path((name, repo)): Path<(String, String)>,
) -> Result<Json<TagList>, StatusCode> {
    let manager = state.registry_manager.read().await;

    if let Some(client) = manager.get_registry(&name) {
        match client.list_tags(&repo).await {
            Ok(tags) => Ok(Json(TagList { name: repo, tags })),
            Err(e) => {
                error!("Failed to list tags for {}/{}: {}", name, repo, e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        error!("Registry not found: {}", name);
        Err(StatusCode::NOT_FOUND)
    }
}

/// Get detailed image information
async fn get_image_info(
    State(state): State<AppState>,
    Path((name, repo, tag)): Path<(String, String, String)>,
) -> Result<Json<ImageInfo>, StatusCode> {
    let manager = state.registry_manager.read().await;

    if let Some(client) = manager.get_registry(&name) {
        match client.get_image_info(&repo, &tag).await {
            Ok(image_info) => Ok(Json(image_info)),
            Err(e) => {
                error!("Failed to get image info for {}/{}:{}: {}", name, repo, tag, e);
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    } else {
        error!("Registry not found: {}", name);
        Err(StatusCode::NOT_FOUND)
    }
}

/// Search for images across registries
async fn search_images(
    State(state): State<AppState>,
    Json(request): Json<ImageSearchRequest>,
) -> Result<Json<ImageSearchResponse>, StatusCode> {
    let manager = state.registry_manager.read().await;

    let results = if let Some(registry_name) = &request.registry {
        // Search in specific registry
        if let Some(client) = manager.get_registry(registry_name) {
            if let Ok(repositories) = client.list_repositories().await {
                let mut images = Vec::new();
                for repo in repositories {
                    if repo.contains(&request.query) {
                        if let Ok(tags) = client.list_tags(&repo).await {
                            for tag in tags {
                                if let Ok(image_info) = client.get_image_info(&repo, &tag).await {
                                    images.push(ImageSearchResult {
                                        registry: registry_name.clone(),
                                        repository: image_info.repository,
                                        tag: image_info.tag,
                                        digest: image_info.digest,
                                        size: image_info.size,
                                        created: image_info.created,
                                    });
                                }
                            }
                        }
                    }
                }
                images
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        // Search across all registries
        match manager.search_images(&request.query).await {
            Ok(results) => results.into_iter().map(|(registry, image_info)| {
                ImageSearchResult {
                    registry,
                    repository: image_info.repository,
                    tag: image_info.tag,
                    digest: image_info.digest,
                    size: image_info.size,
                    created: image_info.created,
                }
            }).collect(),
            Err(e) => {
                error!("Failed to search images: {}", e);
                Vec::new()
            }
        }
    };

    Ok(Json(ImageSearchResponse { images: results }))
}

/// Search for images via GET request (for wizard)
async fn search_images_get(
    State(state): State<AppState>,
    Query(params): Query<ImageSearchQuery>,
) -> Result<Json<Vec<ImageInfo>>, StatusCode> {
    let manager = state.registry_manager.read().await;

    // Convert search results to ImageInfo format expected by wizard
    let results = if let Some(registry_name) = &params.registry {
        // Search in specific registry
        if let Some(client) = manager.get_registry(registry_name) {
            if let Ok(repositories) = client.list_repositories().await {
                let mut images = Vec::new();
                for repo in repositories {
                    if repo.contains(&params.q) {
                        if let Ok(tags) = client.list_tags(&repo).await {
                            for tag in tags {
                                if let Ok(image_info) = client.get_image_info(&repo, &tag).await {
                                    images.push(image_info);
                                }
                            }
                        }
                    }
                }
                images
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        // Search across all registries
        match manager.search_images(&params.q).await {
            Ok(results) => results.into_iter().map(|(_, image_info)| image_info).collect(),
            Err(e) => {
                error!("Failed to search images: {}", e);
                Vec::new()
            }
        }
    };

    Ok(Json(results))
}

/// Pull an image from a registry
async fn pull_image(
    State(state): State<AppState>,
    Json(request): Json<ImagePullRequest>,
) -> Result<Json<OperationResult>, StatusCode> {
    let manager = state.registry_manager.read().await;

    if let Some(client) = manager.get_registry(&request.registry) {
        match client.pull_image(&request.repository, &request.tag).await {
            Ok(_) => {
                info!("Successfully pulled image {}:{} from {}", request.repository, request.tag, request.registry);
                Ok(Json(OperationResult {
                    success: true,
                    message: format!("Successfully pulled {}:{}", request.repository, request.tag),
                }))
            }
            Err(e) => {
                error!("Failed to pull image {}:{} from {}: {}", request.repository, request.tag, request.registry, e);
                Ok(Json(OperationResult {
                    success: false,
                    message: format!("Failed to pull image: {}", e),
                }))
            }
        }
    } else {
        error!("Registry not found: {}", request.registry);
        Ok(Json(OperationResult {
            success: false,
            message: format!("Registry '{}' not found", request.registry),
        }))
    }
}

/// List all containers
async fn list_containers(State(state): State<AppState>) -> Result<Json<ContainerListResponse>, StatusCode> {
    match state.bolt_client.list_containers(None).await {
        Ok(containers) => {
            info!("Retrieved {} containers", containers.len());
            Ok(Json(ContainerListResponse { containers }))
        }
        Err(e) => {
            error!("Failed to list containers: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get detailed container information
async fn get_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Container>, StatusCode> {
    // For mock client, we'll get the container from the list
    match state.bolt_client.list_containers(None).await {
        Ok(containers) => {
            if let Some(container) = containers.into_iter().find(|c| c.id == id) {
                Ok(Json(container))
            } else {
                error!("Container not found: {}", id);
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            error!("Failed to get container {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Create a new container
async fn create_container(
    State(state): State<AppState>,
    Json(request): Json<CreateContainerRequest>,
) -> Result<(StatusCode, Json<OperationResult>), StatusCode> {
    info!("Creating container '{}' with image: {}", request.name, request.image);

    match state.bolt_client.create_container(request).await {
        Ok(container_id) => {
            info!("Created container: {}", container_id);
            Ok((StatusCode::CREATED, Json(OperationResult {
                success: true,
                message: format!("Container created successfully with ID: {}", container_id),
            })))
        }
        Err(e) => {
            error!("Failed to create container: {}", e);
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(OperationResult {
                success: false,
                message: format!("Failed to create container: {}", e),
            })))
        }
    }
}

/// Start a container
async fn start_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<OperationResult>, StatusCode> {
    match state.bolt_client.start_container(&id).await {
        Ok(_) => {
            info!("Started container: {}", id);
            Ok(Json(OperationResult {
                success: true,
                message: format!("Container {} started successfully", id),
            }))
        }
        Err(e) => {
            error!("Failed to start container {}: {}", id, e);
            Ok(Json(OperationResult {
                success: false,
                message: format!("Failed to start container: {}", e),
            }))
        }
    }
}

/// Stop a container
async fn stop_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<ContainerOperationRequest>,
) -> Result<Json<OperationResult>, StatusCode> {
    match state.bolt_client.stop_container(&id, request.timeout).await {
        Ok(_) => {
            info!("Stopped container: {}", id);
            Ok(Json(OperationResult {
                success: true,
                message: format!("Container {} stopped successfully", id),
            }))
        }
        Err(e) => {
            error!("Failed to stop container {}: {}", id, e);
            Ok(Json(OperationResult {
                success: false,
                message: format!("Failed to stop container: {}", e),
            }))
        }
    }
}

/// Restart a container
async fn restart_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<ContainerOperationRequest>,
) -> Result<Json<OperationResult>, StatusCode> {
    match state.bolt_client.restart_container(&id, request.timeout).await {
        Ok(_) => {
            info!("Restarted container: {}", id);
            Ok(Json(OperationResult {
                success: true,
                message: format!("Container {} restarted successfully", id),
            }))
        }
        Err(e) => {
            error!("Failed to restart container {}: {}", id, e);
            Ok(Json(OperationResult {
                success: false,
                message: format!("Failed to restart container: {}", e),
            }))
        }
    }
}

/// Delete a container
async fn delete_container(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<ContainerOperationRequest>,
) -> Result<Json<OperationResult>, StatusCode> {
    let force = request.force.unwrap_or(false);
    let remove_volumes = request.remove_volumes.unwrap_or(false);

    match state.bolt_client.remove_container(&id, force, remove_volumes).await {
        Ok(_) => {
            info!("Removed container: {}", id);
            Ok(Json(OperationResult {
                success: true,
                message: format!("Container {} removed successfully", id),
            }))
        }
        Err(e) => {
            error!("Failed to remove container {}: {}", id, e);
            Ok(Json(OperationResult {
                success: false,
                message: format!("Failed to remove container: {}", e),
            }))
        }
    }
}

/// Get container logs
async fn get_container_logs(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<String, StatusCode> {
    let logs_request = ContainerLogsRequest {
        container_id: id.clone(),
        follow: false,
        tail: Some(100),
        timestamps: true,
        since: None,
    };

    match state.bolt_client.get_container_logs(logs_request).await {
        Ok(logs) => Ok(logs),
        Err(e) => {
            error!("Failed to get logs for container {}: {}", id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get container stats
async fn get_container_stats(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // For mock implementation, return mock stats
    let mock_stats = serde_json::json!({
        "container_id": id,
        "timestamp": chrono::Utc::now(),
        "cpu_percent": 15.2,
        "memory_usage": 134217728, // 128MB
        "memory_limit": 536870912, // 512MB
        "network_rx": 1024000,
        "network_tx": 2048000,
        "block_read": 512000,
        "block_write": 256000,
        "pid_count": 12
    });

    Ok(Json(mock_stats))
}