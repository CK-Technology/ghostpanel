use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="dashboard">
            <div class="stats-grid">
                <div class="container-card">
                    <h3>"Containers"</h3>
                    <div class="stat-value">"12"</div>
                    <div class="stat-label">"4 running, 8 stopped"</div>
                </div>
                <div class="container-card">
                    <h3>"Images"</h3>
                    <div class="stat-value">"8"</div>
                    <div class="stat-label">"2.4 GB total"</div>
                </div>
                <div class="container-card">
                    <h3>"Networks"</h3>
                    <div class="stat-value">"3"</div>
                    <div class="stat-label">"1 gaming network"</div>
                </div>
                <div class="container-card">
                    <h3>"Gaming Containers"</h3>
                    <div class="stat-value">"2"</div>
                    <div class="stat-label">"1 with GPU access"</div>
                </div>
            </div>
        </div>
    }
}