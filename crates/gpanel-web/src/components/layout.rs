use leptos::*;
use leptos_router::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="app-layout">
            <Sidebar/>
            <div class="main-content">
                <Header/>
                <div class="content">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <div class="sidebar">
            <div class="sidebar-header">
                <h2>"🚀 GhostPanel"</h2>
                <p>"Bolt Container Management"</p>
            </div>
            <nav class="sidebar-nav">
                <A href="/" class="nav-item">"📊 Dashboard"</A>
                <A href="/containers" class="nav-item">"📦 Containers"</A>
                <A href="/images" class="nav-item">"🖼️ Images"</A>
                <A href="/networks" class="nav-item">"🌐 Networks"</A>
                <A href="/volumes" class="nav-item">"💾 Volumes"</A>
                <A href="/gaming" class="nav-item">"🎮 Gaming"</A>
                <A href="/settings" class="nav-item">"⚙️ Settings"</A>
            </nav>
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="header">
            <h1>"Container Management"</h1>
            <div class="header-actions">
                <button class="btn-primary">"New Container"</button>
            </div>
        </div>
    }
}