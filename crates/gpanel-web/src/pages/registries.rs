use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

/// Registry configuration response from API (without credentials)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfigResponse {
    pub name: String,
    pub url: String,
    pub has_auth: bool,
    pub insecure: bool,
}

/// Registry list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryListResponse {
    pub registries: Vec<RegistryConfigResponse>,
}

/// Add registry request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRegistryRequest {
    pub name: String,
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub insecure: bool,
}

/// Repository list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryList {
    pub repositories: Vec<String>,
}

/// Tag list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagList {
    pub name: String,
    pub tags: Vec<String>,
}

/// Image information
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerInfo {
    pub digest: String,
    pub size: u64,
    pub media_type: String,
    pub created_by: Option<String>,
}

/// Operation result response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub success: bool,
    pub message: String,
}

/// Format file size in human readable format
fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

#[component]
pub fn RegistryManagement() -> impl IntoView {
    let (registries, set_registries) = create_signal(Vec::<RegistryConfigResponse>::new());
    let (selected_registry, set_selected_registry) = create_signal(None::<String>);
    let (repositories, set_repositories) = create_signal(Vec::<String>::new());
    let (selected_repo, set_selected_repo) = create_signal(None::<String>);
    let (tags, set_tags) = create_signal(Vec::<String>::new());
    let (selected_image_info, set_selected_image_info) = create_signal(None::<ImageInfo>);

    let (show_add_modal, set_show_add_modal) = create_signal(false);
    let (loading, set_loading) = create_signal(false);
    let (error_message, set_error_message) = create_signal(None::<String>);

    // Form fields for adding registry
    let (registry_name, set_registry_name) = create_signal(String::new());
    let (registry_url, set_registry_url) = create_signal(String::new());
    let (registry_username, set_registry_username) = create_signal(String::new());
    let (registry_password, set_registry_password) = create_signal(String::new());
    let (registry_insecure, set_registry_insecure) = create_signal(false);

    // Load registries on mount
    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(response) = Request::get("http://localhost:8000/api/v1/registries")
                .send()
                .await
            {
                if let Ok(registry_list) = response.json::<RegistryListResponse>().await {
                    set_registries.set(registry_list.registries);
                }
            }
        });
    });

    // Load repositories when registry is selected
    create_effect(move |_| {
        if let Some(registry_name) = selected_registry.get() {
            spawn_local(async move {
                set_loading.set(true);
                let url = format!("http://localhost:8000/api/v1/registries/{}/repositories", registry_name);

                match Request::get(&url).send().await {
                    Ok(response) => {
                        if let Ok(repo_list) = response.json::<RepositoryList>().await {
                            set_repositories.set(repo_list.repositories);
                        }
                    }
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to load repositories: {}", e)));
                    }
                }
                set_loading.set(false);
            });
        }
    });

    // Load tags when repository is selected
    create_effect(move |_| {
        if let (Some(registry_name), Some(repo_name)) = (selected_registry.get(), selected_repo.get()) {
            spawn_local(async move {
                set_loading.set(true);
                let url = format!("http://localhost:8000/api/v1/registries/{}/repositories/{}/tags",
                                registry_name, repo_name);

                match Request::get(&url).send().await {
                    Ok(response) => {
                        if let Ok(tag_list) = response.json::<TagList>().await {
                            set_tags.set(tag_list.tags);
                        }
                    }
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to load tags: {}", e)));
                    }
                }
                set_loading.set(false);
            });
        }
    });

    let add_registry = move |_| {
        spawn_local(async move {
            set_loading.set(true);

            let request = AddRegistryRequest {
                name: registry_name.get(),
                url: registry_url.get(),
                username: if registry_username.get().is_empty() { None } else { Some(registry_username.get()) },
                password: if registry_password.get().is_empty() { None } else { Some(registry_password.get()) },
                insecure: registry_insecure.get(),
            };

            match Request::post("http://localhost:8000/api/v1/registries")
                .json(&request)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if let Ok(result) = response.json::<OperationResult>().await {
                        if result.success {
                            // Refresh registry list
                            if let Ok(response) = Request::get("http://localhost:8000/api/v1/registries")
                                .send()
                                .await
                            {
                                if let Ok(registry_list) = response.json::<RegistryListResponse>().await {
                                    set_registries.set(registry_list.registries);
                                }
                            }

                            // Reset form and close modal
                            set_registry_name.set(String::new());
                            set_registry_url.set(String::new());
                            set_registry_username.set(String::new());
                            set_registry_password.set(String::new());
                            set_registry_insecure.set(false);
                            set_show_add_modal.set(false);
                        } else {
                            set_error_message.set(Some(result.message));
                        }
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Failed to add registry: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    let get_image_info = move |tag: String| {
        if let (Some(registry_name), Some(repo_name)) = (selected_registry.get(), selected_repo.get()) {
            spawn_local(async move {
                set_loading.set(true);
                let url = format!("http://localhost:8000/api/v1/registries/{}/repositories/{}/tags/{}",
                                registry_name, repo_name, tag);

                match Request::get(&url).send().await {
                    Ok(response) => {
                        if let Ok(image_info) = response.json::<ImageInfo>().await {
                            set_selected_image_info.set(Some(image_info));
                        }
                    }
                    Err(e) => {
                        set_error_message.set(Some(format!("Failed to load image info: {}", e)));
                    }
                }
                set_loading.set(false);
            });
        }
    };

    view! {
        <div class="registry-management">
            <div class="header-section">
                <h2>"Registry Management"</h2>
                <p>"Manage container image registries including Docker Hub and Drift"</p>
                <button class="btn-primary" on:click=move |_| set_show_add_modal.set(true)>
                    "Add Registry"
                </button>
            </div>

            // Error message display
            {move || {
                if let Some(error) = error_message.get() {
                    view! {
                        <div class="error-banner" style="background-color: #e74c3c; color: white; padding: 10px; border-radius: 4px; margin-bottom: 20px;">
                            {error}
                            <button style="float: right; background: none; border: none; color: white; cursor: pointer;"
                                    on:click=move |_| set_error_message.set(None)>
                                "×"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px; margin-bottom: 30px;">
                // Registry List
                <div class="container-card">
                    <h3>"Registries"</h3>
                    <div style="max-height: 400px; overflow-y: auto;">
                        <For
                            each=move || registries.get()
                            key=|registry| registry.name.clone()
                            children=move |registry| {
                                let registry_name = registry.name.clone();
                                let registry_name_for_click = registry_name.clone();
                                let is_selected = move || selected_registry.get() == Some(registry_name.clone());

                                view! {
                                    <div
                                        class="registry-item"
                                        style=move || format!(
                                            "padding: 10px; margin: 5px 0; border-radius: 4px; cursor: pointer; {}",
                                            if is_selected() { "background-color: #3498db; color: white;" } else { "background-color: #34495e;" }
                                        )
                                        on:click=move |_| {
                                            set_selected_registry.set(Some(registry_name_for_click.clone()));
                                            set_selected_repo.set(None);
                                            set_tags.set(Vec::new());
                                            set_selected_image_info.set(None);
                                        }
                                    >
                                        <div style="font-weight: bold;">{&registry.name}</div>
                                        <div style="font-size: 12px; opacity: 0.8;">{&registry.url}</div>
                                        {if registry.has_auth {
                                            view! { <span style="font-size: 10px; background-color: #27ae60; padding: 2px 4px; border-radius: 2px;">
                                                "AUTH"
                                            </span> }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }}
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>

                // Repository List
                <div class="container-card">
                    <h3>"Repositories"</h3>
                    {move || {
                        if selected_registry.get().is_some() {
                            view! {
                                <div style="max-height: 400px; overflow-y: auto;">
                                    <For
                                        each=move || repositories.get()
                                        key=|repo| repo.clone()
                                        children=move |repo| {
                                            let repo_name = repo.clone();
                                            let repo_name_for_click = repo_name.clone();
                                            let is_selected = move || selected_repo.get() == Some(repo_name.clone());

                                            view! {
                                                <div
                                                    class="repo-item"
                                                    style=move || format!(
                                                        "padding: 8px; margin: 3px 0; border-radius: 4px; cursor: pointer; font-size: 14px; {}",
                                                        if is_selected() { "background-color: #3498db; color: white;" } else { "background-color: #34495e;" }
                                                    )
                                                    on:click=move |_| {
                                                        set_selected_repo.set(Some(repo_name_for_click.clone()));
                                                        set_selected_image_info.set(None);
                                                    }
                                                >
                                                    {repo}
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div style="text-align: center; color: #888; padding: 20px;">
                                    "Select a registry to view repositories"
                                </div>
                            }.into_view()
                        }
                    }}
                </div>

                // Tag List
                <div class="container-card">
                    <h3>"Tags"</h3>
                    {move || {
                        if selected_repo.get().is_some() {
                            view! {
                                <div style="max-height: 400px; overflow-y: auto;">
                                    <For
                                        each=move || tags.get()
                                        key=|tag| tag.clone()
                                        children=move |tag| {
                                            let tag_name = tag.clone();

                                            view! {
                                                <div
                                                    class="tag-item"
                                                    style="padding: 8px; margin: 3px 0; border-radius: 4px; cursor: pointer; font-size: 14px; background-color: #34495e; display: flex; justify-content: space-between; align-items: center;"
                                                    on:click=move |_| get_image_info(tag_name.clone())
                                                >
                                                    <span>{tag}</span>
                                                    <button class="btn-primary" style="padding: 4px 8px; font-size: 12px;">
                                                        "Inspect"
                                                    </button>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            }.into_view()
                        } else {
                            view! {
                                <div style="text-align: center; color: #888; padding: 20px;">
                                    "Select a repository to view tags"
                                </div>
                            }.into_view()
                        }
                    }}
                </div>
            </div>

            // Image Details Panel
            {move || {
                if let Some(image_info) = selected_image_info.get() {
                    view! {
                        <div class="container-card">
                            <h3>"Image Details: " {&image_info.repository} ":" {&image_info.tag}</h3>

                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 20px; margin-top: 20px;">
                                <div>
                                    <h4>"Metadata"</h4>
                                    <div style="margin: 10px 0;">
                                        <strong>"Size: "</strong> {format_size(image_info.size)}
                                    </div>
                                    <div style="margin: 10px 0;">
                                        <strong>"Created: "</strong> {image_info.created.format("%Y-%m-%d %H:%M:%S UTC").to_string()}
                                    </div>
                                    <div style="margin: 10px 0;">
                                        <strong>"Digest: "</strong>
                                        <code style="background-color: #1a1a1a; padding: 2px 4px; border-radius: 2px; font-size: 12px;">
                                            {&image_info.digest}
                                        </code>
                                    </div>
                                    {if let Some(author) = &image_info.author {
                                        view! {
                                            <div style="margin: 10px 0;">
                                                <strong>"Author: "</strong> {author}
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! { <div></div> }.into_view()
                                    }}
                                </div>

                                <div>
                                    <h4>{format!("Layers ({})", image_info.layers.len())}</h4>
                                    <div style="max-height: 200px; overflow-y: auto;">
                                        <For
                                            each=move || image_info.layers.clone()
                                            key=|layer| layer.digest.clone()
                                            children=move |layer| {
                                                view! {
                                                    <div style="background-color: #1a1a1a; padding: 8px; margin: 4px 0; border-radius: 4px; font-size: 12px;">
                                                        <div>
                                                            <code>{layer.digest.split(':').last().unwrap_or(&layer.digest)[..12].to_string()}</code>
                                                            <span style="float: right;">{format_size(layer.size)}</span>
                                                        </div>
                                                        <div style="color: #888; margin-top: 4px;">
                                                            {&layer.media_type}
                                                        </div>
                                                    </div>
                                                }
                                            }
                                        />
                                    </div>
                                </div>
                            </div>

                            <div style="margin-top: 20px;">
                                <button class="btn-success" style="margin-right: 10px;">
                                    "Pull Image"
                                </button>
                                <button class="btn-primary">
                                    "Create Container"
                                </button>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Add Registry Modal
            {move || {
                if show_add_modal.get() {
                    view! {
                        <div style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;">
                            <div class="container-card" style="width: 500px; max-width: 90vw;">
                                <h3>"Add Registry"</h3>

                                <div style="margin: 15px 0;">
                                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Registry Name"</label>
                                    <input
                                        type="text"
                                        placeholder="my-registry"
                                        style="width: 100%; padding: 8px; border: 1px solid #555; border-radius: 4px; background-color: #2c3e50; color: white;"
                                        prop:value=move || registry_name.get()
                                        on:input=move |ev| set_registry_name.set(event_target_value(&ev))
                                    />
                                </div>

                                <div style="margin: 15px 0;">
                                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Registry URL"</label>
                                    <input
                                        type="url"
                                        placeholder="https://registry.example.com or http://localhost:5000"
                                        style="width: 100%; padding: 8px; border: 1px solid #555; border-radius: 4px; background-color: #2c3e50; color: white;"
                                        prop:value=move || registry_url.get()
                                        on:input=move |ev| set_registry_url.set(event_target_value(&ev))
                                    />
                                </div>

                                <div style="margin: 15px 0;">
                                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Username (optional)"</label>
                                    <input
                                        type="text"
                                        placeholder="username"
                                        style="width: 100%; padding: 8px; border: 1px solid #555; border-radius: 4px; background-color: #2c3e50; color: white;"
                                        prop:value=move || registry_username.get()
                                        on:input=move |ev| set_registry_username.set(event_target_value(&ev))
                                    />
                                </div>

                                <div style="margin: 15px 0;">
                                    <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Password (optional)"</label>
                                    <input
                                        type="password"
                                        placeholder="password"
                                        style="width: 100%; padding: 8px; border: 1px solid #555; border-radius: 4px; background-color: #2c3e50; color: white;"
                                        prop:value=move || registry_password.get()
                                        on:input=move |ev| set_registry_password.set(event_target_value(&ev))
                                    />
                                </div>

                                <div style="margin: 15px 0;">
                                    <label style="display: flex; align-items: center;">
                                        <input
                                            type="checkbox"
                                            style="margin-right: 8px;"
                                            prop:checked=move || registry_insecure.get()
                                            on:change=move |ev| set_registry_insecure.set(event_target_checked(&ev))
                                        />
                                        "Allow insecure connections (HTTP)"
                                    </label>
                                </div>

                                <div style="display: flex; justify-content: flex-end; gap: 10px; margin-top: 20px;">
                                    <button
                                        class="btn-primary"
                                        style="background-color: #555;"
                                        on:click=move |_| set_show_add_modal.set(false)
                                    >
                                        "Cancel"
                                    </button>
                                    <button
                                        class="btn-primary"
                                        on:click=add_registry
                                        disabled=move || loading.get()
                                    >
                                        {move || if loading.get() { "Adding..." } else { "Add Registry" }}
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}

            // Loading indicator
            {move || {
                if loading.get() {
                    view! {
                        <div style="position: fixed; top: 20px; right: 20px; background-color: #3498db; color: white; padding: 10px 20px; border-radius: 4px; z-index: 1500;">
                            "Loading..."
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }}
        </div>
    }
}