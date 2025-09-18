use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Registry configuration for connecting to Docker/Drift registries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    pub name: String,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub insecure: bool,
}

/// Registry client for interacting with Docker Registry v2 API and Drift extensions
#[derive(Debug, Clone)]
pub struct RegistryClient {
    client: Client,
    config: RegistryConfig,
    auth_token: Option<String>,
}

/// Container image manifest as returned by registry API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageManifest {
    pub schema_version: i32,
    pub media_type: String,
    pub config: Descriptor,
    pub layers: Vec<Descriptor>,
}

/// Image descriptor containing metadata about layers and configs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub media_type: String,
    pub size: u64,
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// Repository list response from catalog API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryList {
    pub repositories: Vec<String>,
}

/// Tag list response for a specific repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagList {
    pub name: String,
    pub tags: Vec<String>,
}

/// Image information with metadata for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub repository: String,
    pub tag: String,
    pub digest: String,
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
    pub author: Option<String>,
    pub layers: Vec<LayerInfo>,
}

/// Layer information for image inspection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerInfo {
    pub digest: String,
    pub size: u64,
    pub media_type: String,
    pub created_by: Option<String>,
}

impl RegistryClient {
    /// Create a new registry client
    pub fn new(config: RegistryConfig) -> Self {
        let client = Client::new();
        Self {
            client,
            config,
            auth_token: None,
        }
    }

    /// Authenticate with the registry if credentials are provided
    pub async fn authenticate(&mut self) -> Result<()> {
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            // For Docker Registry v2, we need to get a token from the auth endpoint
            let auth_url = format!("{}/v2/", self.config.url);

            debug!("Authenticating with registry: {}", self.config.url);

            // First, try to access the registry to get the auth challenge
            let response = self.client.get(&auth_url).send().await?;

            if response.status() == 401 {
                // Parse WWW-Authenticate header to get auth service info
                if let Some(auth_header) = response.headers().get("www-authenticate") {
                    let auth_str = auth_header.to_str().context("Invalid auth header")?;

                    // Parse Bearer realm, service, scope from header
                    if let Some(token) = self.get_auth_token(auth_str, username, password).await? {
                        self.auth_token = Some(token);
                        info!("Successfully authenticated with registry: {}", self.config.name);
                    }
                }
            }
        }
        Ok(())
    }

    /// Get authentication token from auth service
    async fn get_auth_token(&self, auth_header: &str, username: &str, password: &str) -> Result<Option<String>> {
        // Parse auth header: Bearer realm="...", service="...", scope="..."
        let mut realm = None;
        let mut service = None;

        let header_without_bearer = auth_header.replace("Bearer ", "");
        for part in header_without_bearer.split(',') {
            let part = part.trim();
            if let Some(value) = part.strip_prefix("realm=") {
                realm = Some(value.trim_matches('"'));
            } else if let Some(value) = part.strip_prefix("service=") {
                service = Some(value.trim_matches('"'));
            }
        }

        if let (Some(realm), Some(service)) = (realm, service) {
            let auth_url = format!("{}?service={}&scope=registry:catalog:*", realm, service);

            let response = self.client
                .get(&auth_url)
                .basic_auth(username, Some(password))
                .send()
                .await?;

            if response.status().is_success() {
                #[derive(Deserialize)]
                struct TokenResponse {
                    token: String,
                }

                let token_resp: TokenResponse = response.json().await?;
                return Ok(Some(token_resp.token));
            }
        }

        Ok(None)
    }

    /// List all repositories in the registry
    pub async fn list_repositories(&self) -> Result<Vec<String>> {
        let url = format!("{}/v2/_catalog", self.config.url);

        let mut request = self.client.get(&url);
        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to list repositories: {}", response.status()));
        }

        let repo_list: RepositoryList = response.json().await?;
        Ok(repo_list.repositories)
    }

    /// List tags for a specific repository
    pub async fn list_tags(&self, repository: &str) -> Result<Vec<String>> {
        let url = format!("{}/v2/{}/tags/list", self.config.url, repository);

        let mut request = self.client.get(&url);
        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to list tags for {}: {}", repository, response.status()));
        }

        let tag_list: TagList = response.json().await?;
        Ok(tag_list.tags)
    }

    /// Get manifest for a specific image
    pub async fn get_manifest(&self, repository: &str, tag: &str) -> Result<ImageManifest> {
        let url = format!("{}/v2/{}/manifests/{}", self.config.url, repository, tag);

        let mut request = self.client.get(&url)
            .header("Accept", "application/vnd.docker.distribution.manifest.v2+json");

        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Failed to get manifest for {}:{}: {}", repository, tag, response.status()));
        }

        let manifest: ImageManifest = response.json().await?;
        Ok(manifest)
    }

    /// Get detailed image information including layers and metadata
    pub async fn get_image_info(&self, repository: &str, tag: &str) -> Result<ImageInfo> {
        let manifest = self.get_manifest(repository, tag).await?;

        // Calculate total size from layers
        let total_size: u64 = manifest.layers.iter().map(|l| l.size).sum();

        // Get image config to extract creation date and other metadata
        let config_url = format!("{}/v2/{}/blobs/{}", self.config.url, repository, manifest.config.digest);

        let mut request = self.client.get(&config_url);
        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        let config_response = request.send().await?;
        let config_data: serde_json::Value = config_response.json().await?;

        // Extract created timestamp and author from config
        let created = config_data
            .get("created")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);

        let author = config_data
            .get("author")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        // Convert layers to LayerInfo
        let layers: Vec<LayerInfo> = manifest.layers.into_iter().map(|layer| {
            LayerInfo {
                digest: layer.digest,
                size: layer.size,
                media_type: layer.media_type,
                created_by: None, // Would need to parse history from config for this
            }
        }).collect();

        Ok(ImageInfo {
            repository: repository.to_string(),
            tag: tag.to_string(),
            digest: manifest.config.digest,
            size: total_size,
            created,
            author,
            layers,
        })
    }

    /// Pull an image (download layers) - simplified for now
    pub async fn pull_image(&self, repository: &str, tag: &str) -> Result<()> {
        info!("Pulling image {}:{}", repository, tag);

        let manifest = self.get_manifest(repository, tag).await?;

        // In a real implementation, we would download and store the layers
        // For now, we'll just verify they exist
        for layer in &manifest.layers {
            let blob_url = format!("{}/v2/{}/blobs/{}", self.config.url, repository, layer.digest);

            let mut request = self.client.head(&blob_url);
            if let Some(token) = &self.auth_token {
                request = request.bearer_auth(token);
            }

            let response = request.send().await?;
            if !response.status().is_success() {
                return Err(anyhow::anyhow!("Layer {} not found", layer.digest));
            }
        }

        info!("Successfully verified image {}:{}", repository, tag);
        Ok(())
    }

    /// Push an image (upload layers and manifest) - placeholder
    pub async fn push_image(&self, _repository: &str, _tag: &str) -> Result<()> {
        // This would require implementing the full Docker Registry v2 push protocol
        // Including blob uploads, manifest uploads, etc.
        Err(anyhow::anyhow!("Push functionality not yet implemented"))
    }

    /// Delete an image from the registry
    pub async fn delete_image(&self, repository: &str, tag: &str) -> Result<()> {
        // First get the manifest to get the digest for deletion
        let url = format!("{}/v2/{}/manifests/{}", self.config.url, repository, tag);

        let mut request = self.client.get(&url)
            .header("Accept", "application/vnd.docker.distribution.manifest.v2+json");

        if let Some(token) = &self.auth_token {
            request = request.bearer_auth(token);
        }

        let response = request.send().await?;

        if let Some(digest) = response.headers().get("docker-content-digest") {
            let digest_str = digest.to_str().context("Invalid digest header")?;

            // Delete by digest
            let delete_url = format!("{}/v2/{}/manifests/{}", self.config.url, repository, digest_str);

            let mut delete_request = self.client.delete(&delete_url);
            if let Some(token) = &self.auth_token {
                delete_request = delete_request.bearer_auth(token);
            }

            let delete_response = delete_request.send().await?;

            if delete_response.status().is_success() {
                info!("Successfully deleted image {}:{}", repository, tag);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Failed to delete image: {}", delete_response.status()))
            }
        } else {
            Err(anyhow::anyhow!("Could not get digest for image deletion"))
        }
    }
}

/// Registry manager for handling multiple registries
#[derive(Debug)]
pub struct RegistryManager {
    registries: HashMap<String, RegistryClient>,
}

impl RegistryManager {
    pub fn new() -> Self {
        Self {
            registries: HashMap::new(),
        }
    }

    /// Add a new registry configuration
    pub async fn add_registry(&mut self, config: RegistryConfig) -> Result<()> {
        let mut client = RegistryClient::new(config.clone());
        client.authenticate().await?;
        self.registries.insert(config.name.clone(), client);
        Ok(())
    }

    /// Get a registry client by name
    pub fn get_registry(&self, name: &str) -> Option<&RegistryClient> {
        self.registries.get(name)
    }

    /// List all configured registries
    pub fn list_registries(&self) -> Vec<&str> {
        self.registries.keys().map(|s| s.as_str()).collect()
    }

    /// Remove a registry
    pub fn remove_registry(&mut self, name: &str) -> bool {
        self.registries.remove(name).is_some()
    }

    /// Search for images across all registries
    pub async fn search_images(&self, query: &str) -> Result<Vec<(String, ImageInfo)>> {
        let mut results = Vec::new();

        for (registry_name, client) in &self.registries {
            if let Ok(repositories) = client.list_repositories().await {
                for repo in repositories {
                    if repo.contains(query) {
                        if let Ok(tags) = client.list_tags(&repo).await {
                            for tag in tags {
                                if let Ok(image_info) = client.get_image_info(&repo, &tag).await {
                                    results.push((registry_name.clone(), image_info));
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}

impl Default for RegistryManager {
    fn default() -> Self {
        Self::new()
    }
}