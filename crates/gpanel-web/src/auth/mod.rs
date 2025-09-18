pub mod oidc;

use leptos::*;
use serde::{Deserialize, Serialize};

pub use oidc::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user: RwSignal<Option<User>>,
    pub token: RwSignal<Option<String>>,
}

impl AuthContext {
    pub fn new() -> Self {
        Self {
            user: create_rw_signal(None),
            token: create_rw_signal(None),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.user.get().is_some() && self.token.get().is_some()
    }

    pub fn login(&self, user: User, token: String) {
        self.user.set(Some(user));
        self.token.set(Some(token));
    }

    pub fn logout(&self) {
        self.user.set(None);
        self.token.set(None);
    }
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let auth_context = AuthContext::new();
    provide_context(auth_context);
    children()
}