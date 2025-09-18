use leptos::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    view! {
        <div class="login-page">
            <div class="login-container">
                <div class="login-header">
                    <h1>"🚀 GhostPanel"</h1>
                    <p>"Sign in to manage your Bolt containers"</p>
                </div>
                <div class="login-form">
                    <button class="btn-primary">"Sign in with Local Account"</button>
                    <div class="divider">"or"</div>
                    <button class="btn-primary">"Sign in with Azure AD"</button>
                    <button class="btn-primary">"Sign in with Google"</button>
                    <button class="btn-primary">"Sign in with GitHub"</button>
                </div>
            </div>
        </div>
    }
}