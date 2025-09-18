use leptos::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

/// Image search request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSearchRequest {
    pub query: String,
    pub registry: Option<String>,
}

/// Image search response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSearchResponse {
    pub images: Vec<ImageSearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSearchResult {
    pub registry: String,
    pub repository: String,
    pub tag: String,
    pub digest: String,
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
}

/// Registry configuration response
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

/// Image pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePullRequest {
    pub registry: String,
    pub repository: String,
    pub tag: String,
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
pub fn ImageList() -> impl IntoView {
    let (search_query, set_search_query) = create_signal(String::new());
    let (selected_registry, set_selected_registry) = create_signal(None::<String>);
    let (search_results, set_search_results) = create_signal(Vec::<ImageSearchResult>::new());
    let (registries, set_registries) = create_signal(Vec::<RegistryConfigResponse>::new());
    let (loading, set_loading) = create_signal(false);
    let (error_message, set_error_message) = create_signal(None::<String>);

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

    let search_images = move |_| {
        let query = search_query.get();
        if query.trim().is_empty() {
            return;
        }

        spawn_local(async move {
            set_loading.set(true);
            set_error_message.set(None);

            let request = ImageSearchRequest {
                query: query.clone(),
                registry: selected_registry.get(),
            };

            match Request::post("http://localhost:8000/api/v1/images/search")
                .json(&request)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if let Ok(search_response) = response.json::<ImageSearchResponse>().await {
                        set_search_results.set(search_response.images);
                    } else {
                        set_error_message.set(Some("Failed to parse search results".to_string()));
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("Search failed: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    let pull_image = move |registry: String, repository: String, tag: String| {
        spawn_local(async move {
            set_loading.set(true);

            let request = ImagePullRequest {
                registry,
                repository: repository.clone(),
                tag: tag.clone(),
            };

            match Request::post("http://localhost:8000/api/v1/images/pull")
                .json(&request)
                .unwrap()
                .send()
                .await
            {
                Ok(response) => {
                    if let Ok(result) = response.json::<OperationResult>().await {
                        if result.success {
                            set_error_message.set(Some(format!("✅ Successfully pulled {}:{}", repository, tag)));
                        } else {
                            set_error_message.set(Some(format!("❌ {}", result.message)));
                        }
                    }
                }
                Err(e) => {
                    set_error_message.set(Some(format!("❌ Pull failed: {}", e)));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="image-list">
            <div class="header-section">
                <h2>"Container Images"</h2>
                <p>"Search and manage container images across registries"</p>
            </div>

            // Error/Success message display
            {move || {
                if let Some(message) = error_message.get() {
                    let is_success = message.starts_with("✅");
                    view! {
                        <div
                            class="message-banner"
                            style=format!(
                                "background-color: {}; color: white; padding: 10px; border-radius: 4px; margin-bottom: 20px;",
                                if is_success { "#27ae60" } else { "#e74c3c" }
                            )
                        >
                            {message}
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

            // Search Section
            <div class="container-card" style="margin-bottom: 20px;">
                <h3>"Search Images"</h3>

                <div style="display: grid; grid-template-columns: 1fr 200px auto; gap: 15px; align-items: end; margin-top: 15px;">
                    <div>
                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Search Query"</label>
                        <input
                            type="text"
                            placeholder="nginx, alpine, ubuntu, etc."
                            style="width: 100%; padding: 10px; border: 1px solid #555; border-radius: 4px; background-color: #2c3e50; color: white;"
                            prop:value=move || search_query.get()
                            on:input=move |ev| set_search_query.set(event_target_value(&ev))
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" {
                                    search_images(());
                                }
                            }
                        />
                    </div>

                    <div>
                        <label style="display: block; margin-bottom: 5px; font-weight: bold;">"Registry Filter"</label>
                        <select
                            style="width: 100%; padding: 10px; border: 1px solid #555; border-radius: 4px; background-color: #2c3e50; color: white;"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_selected_registry.set(if value.is_empty() { None } else { Some(value) });
                            }
                        >
                            <option value="">"All Registries"</option>
                            <For
                                each=move || registries.get()
                                key=|registry| registry.name.clone()
                                children=move |registry| {
                                    view! {
                                        <option value={&registry.name}>{&registry.name}</option>
                                    }
                                }
                            />
                        </select>
                    </div>

                    <button
                        class="btn-primary"
                        style="padding: 10px 20px;"
                        on:click=move |_| search_images(())
                        disabled=move || loading.get()
                    >
                        {move || if loading.get() { "Searching..." } else { "Search" }}
                    </button>
                </div>
            </div>

            // Search Results
            <div class="container-card">
                <h3>"Search Results"</h3>

                {move || {
                    let results = search_results.get();
                    if results.is_empty() {
                        view! {
                            <div style="text-align: center; color: #888; padding: 40px;">
                                {if search_query.get().is_empty() {
                                    "Enter a search term to find images"
                                } else {
                                    "No images found. Try a different search term or check registry availability."
                                }}
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div style="margin-top: 20px;">
                                <div style="display: grid; gap: 15px;">
                                    <For
                                        each=move || search_results.get()
                                        key=|image| format!("{}:{}:{}", image.registry, image.repository, image.tag)
                                        children=move |image| {
                                            let registry = image.registry.clone();
                                            let repository = image.repository.clone();
                                            let tag = image.tag.clone();
                                            let registry_for_pull = registry.clone();
                                            let repository_for_pull = repository.clone();
                                            let tag_for_pull = tag.clone();
                                            let repository_for_create = repository.clone();
                                            let tag_for_create = tag.clone();

                                            view! {
                                                <div class="image-item" style="background-color: #34495e; border-radius: 8px; padding: 20px; border: 1px solid #4a5568;">
                                                    <div style="display: grid; grid-template-columns: 1fr auto; gap: 20px; align-items: center;">
                                                        <div>
                                                            <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 10px;">
                                                                <h4 style="margin: 0; color: #3498db;">
                                                                    {&image.repository} ":" {&image.tag}
                                                                </h4>
                                                                <span style="background-color: #2c3e50; padding: 4px 8px; border-radius: 4px; font-size: 12px; color: #bbb;">
                                                                    {&image.registry}
                                                                </span>
                                                            </div>

                                                            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: 10px; font-size: 14px; color: #bbb;">
                                                                <div>
                                                                    <strong>"Size: "</strong> {format_size(image.size)}
                                                                </div>
                                                                <div>
                                                                    <strong>"Created: "</strong> {image.created.format("%Y-%m-%d").to_string()}
                                                                </div>
                                                            </div>

                                                            <div style="margin-top: 8px; font-size: 12px; color: #888;">
                                                                <code style="background-color: #1a1a1a; padding: 2px 4px; border-radius: 2px;">
                                                                    {image.digest.split(':').last().unwrap_or(&image.digest)[..12].to_string()}
                                                                </code>
                                                            </div>
                                                        </div>

                                                        <div style="display: flex; flex-direction: column; gap: 8px;">
                                                            <button
                                                                class="btn-success"
                                                                style="padding: 8px 16px; white-space: nowrap;"
                                                                on:click=move |_| pull_image(registry_for_pull.clone(), repository_for_pull.clone(), tag_for_pull.clone())
                                                                disabled=move || loading.get()
                                                            >
                                                                "Pull"
                                                            </button>
                                                            <button
                                                                class="btn-primary"
                                                                style="padding: 8px 16px; white-space: nowrap;"
                                                                on:click=move |_| {
                                                                    // TODO: Navigate to create container with this image pre-selected
                                                                    web_sys::console::log_1(&format!("Create container from {}:{}", repository_for_create, tag_for_create).into());
                                                                }
                                                            >
                                                                "Create Container"
                                                            </button>
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    />
                                </div>
                            </div>
                        }.into_view()
                    }
                }}
            </div>

            // Quick Actions
            <div class="container-card" style="margin-top: 20px;">
                <h3>"Quick Actions"</h3>
                <div style="display: flex; gap: 15px; margin-top: 15px;">
                    <button
                        class="btn-primary"
                        on:click=move |_| {
                            // Navigate to registry management
                            let navigate = leptos_router::use_navigate();
                            navigate("/registries", Default::default());
                        }
                    >
                        "Manage Registries"
                    </button>
                    <button
                        class="btn-primary"
                        on:click=move |_| {
                            set_search_query.set("alpine".to_string());
                            search_images(());
                        }
                    >
                        "Search Alpine Images"
                    </button>
                    <button
                        class="btn-primary"
                        on:click=move |_| {
                            set_search_query.set("nginx".to_string());
                            search_images(());
                        }
                    >
                        "Search Nginx Images"
                    </button>
                </div>
            </div>

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