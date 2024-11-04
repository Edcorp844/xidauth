use oxide_auth::endpoint::OwnerConsent;
use oxide_auth::frontends::actix::{OAuthRequest, OAuthResponse};
use oxide_auth::primitives::issuer::{TokenMap, Issuer};
use oxide_auth::primitives::registrar::{ClientMap, ExactUrl, Registrar};
use oxide_auth::primitives::scope::Scope;
use oxide_auth::endpoint::{Endpoint, Generic, OAuthError};
use oxide_auth::primitives::grant::Grant;

pub struct AuthConfig {
    pub endpoint: Generic<ClientMap, TokenMap<chrono::Utc>>,
}

impl AuthConfig {
    pub fn new() -> Self {
        let mut clients = ClientMap::new();
        // Add clients to the registrar (replace client_id, client_secret, redirect_uri as needed)
        clients.register_client(
            "client_id",
            "client_secret",
            ExactUrl::from("http://localhost:8080/callback").unwrap(),
            Scope::new(),
        );

        let tokens = TokenMap::new();

        let endpoint = Generic {
            registrar: clients,
            authorizer: None,
            issuer: tokens,
            scopes: Scope::new(),
            registrar_demand: OwnerConsent::Implicit,
        };

        AuthConfig { endpoint }
    }
}
