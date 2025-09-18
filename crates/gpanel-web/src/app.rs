use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::auth::{AuthProvider, AuthContext};
use crate::pages::{
    dashboard::Dashboard,
    containers::ContainerList,
    images::ImageList,
    networks::NetworkList,
    volumes::VolumeList,
    gaming::GamingDashboard,
    login::LoginPage,
    settings::SettingsPage,
};
use crate::components::layout::Layout;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html class="dark"/>
        <Title text="GhostPanel - Bolt Container Management"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>
        <Link rel="icon" type_="image/x-icon" href="/favicon.ico"/>

        // Custom CSS for Portainer-like styling
        <Style>
            "
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
            }

            body {
                background-color: #1a1a1a;
                color: #ffffff;
                font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
                overflow-x: hidden;
            }

            .sidebar {
                width: 250px;
                min-height: 100vh;
                background: linear-gradient(180deg, #2c3e50 0%, #34495e 100%);
                position: fixed;
                left: 0;
                top: 0;
                z-index: 1000;
                border-right: 1px solid #34495e;
            }

            .main-content {
                margin-left: 250px;
                min-height: 100vh;
                background-color: #1a1a1a;
                padding: 20px;
            }

            .container-card {
                background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
                border: 1px solid #4a5568;
                border-radius: 8px;
                padding: 20px;
                margin-bottom: 20px;
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.3);
            }

            .btn-primary {
                background: linear-gradient(135deg, #3498db 0%, #2980b9 100%);
                border: none;
                color: white;
                padding: 10px 20px;
                border-radius: 4px;
                cursor: pointer;
                transition: all 0.2s;
            }

            .btn-primary:hover {
                background: linear-gradient(135deg, #2980b9 0%, #3498db 100%);
                transform: translateY(-1px);
            }

            .btn-success {
                background: linear-gradient(135deg, #27ae60 0%, #2ecc71 100%);
                border: none;
                color: white;
                padding: 10px 20px;
                border-radius: 4px;
                cursor: pointer;
            }

            .btn-danger {
                background: linear-gradient(135deg, #e74c3c 0%, #c0392b 100%);
                border: none;
                color: white;
                padding: 10px 20px;
                border-radius: 4px;
                cursor: pointer;
            }

            .status-running {
                color: #2ecc71;
                font-weight: bold;
            }

            .status-stopped {
                color: #e74c3c;
                font-weight: bold;
            }

            .status-paused {
                color: #f39c12;
                font-weight: bold;
            }

            .gaming-badge {
                background: linear-gradient(135deg, #9b59b6 0%, #8e44ad 100%);
                color: white;
                padding: 4px 8px;
                border-radius: 4px;
                font-size: 12px;
                font-weight: bold;
                margin-left: 8px;
            }

            .gpu-indicator {
                background: linear-gradient(135deg, #f39c12 0%, #e67e22 100%);
                color: white;
                padding: 2px 6px;
                border-radius: 3px;
                font-size: 10px;
                margin-left: 4px;
            }
            "
        </Style>

        <AuthProvider>
            <Router>
                <AuthGuard/>
            </Router>
        </AuthProvider>
    }
}

#[component]
pub fn AuthGuard() -> impl IntoView {
    let auth_context = use_context::<AuthContext>()
        .expect("AuthContext must be provided");

    view! {
        <Show
            when=move || auth_context.is_authenticated()
            fallback=|| view! { <LoginPage/> }
        >
            <Layout>
                <Routes>
                    // Main Dashboard
                    <Route path="/" view=Dashboard/>

                    // Container Management
                    <Route path="/containers" view=ContainerList/>
                    <Route path="/containers/:id" view=|| view! { <div>"Container Details"</div> }/>

                    // Image Management
                    <Route path="/images" view=ImageList/>
                    <Route path="/images/:id" view=|| view! { <div>"Image Details"</div> }/>

                    // Network Management
                    <Route path="/networks" view=NetworkList/>
                    <Route path="/networks/:id" view=|| view! { <div>"Network Details"</div> }/>

                    // Volume Management
                    <Route path="/volumes" view=VolumeList/>
                    <Route path="/volumes/:id" view=|| view! { <div>"Volume Details"</div> }/>

                    // Gaming Features
                    <Route path="/gaming" view=GamingDashboard/>
                    <Route path="/gaming/gpu" view=|| view! { <div>"GPU Management"</div> }/>
                    <Route path="/gaming/proton" view=|| view! { <div>"Proton Manager"</div> }/>
                    <Route path="/gaming/steam" view=|| view! { <div>"Steam Integration"</div> }/>

                    // System & Settings
                    <Route path="/settings" view=SettingsPage/>
                    <Route path="/users" view=|| view! { <div>"User Management"</div> }/>
                    <Route path="/logs" view=|| view! { <div>"System Logs"</div> }/>

                    // Catch-all 404
                    <Route path="/*any" view=|| view! {
                        <div class="container-card">
                            <h1>"404 - Page Not Found"</h1>
                            <p>"The page you're looking for doesn't exist."</p>
                            <a href="/" class="btn-primary">"Go to Dashboard"</a>
                        </div>
                    }/>
                </Routes>
            </Layout>
        </Show>
    }
}