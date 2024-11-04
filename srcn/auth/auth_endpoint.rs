use actix_web::{web, HttpRequest, HttpResponse, Responder};
use oxide_auth::frontends::actix::{OAuthRequest, OAuthResponse};
use oxide_auth::endpoint::OAuthError;
use crate::auth::auth_config::AuthConfig; // Assuming AuthConfig is defined here

pub struct AuthEndpoint {
    auth_config: AuthConfig,
}

impl AuthEndpoint {
    pub fn new() -> Self {
        AuthEndpoint {
            auth_config: AuthConfig::new(),
        }
    }

    // Authorization endpoint to initiate OAuth flow (Auth Code, Implicit, etc.)
    pub async fn authorization(
        &self,
        req: HttpRequest,
        query: web::Query<std::collections::HashMap<String, String>>,
    ) -> impl Responder {
        let mut req = OAuthRequest::from_actix(&req, query.into_inner());
        let response = self.auth_config.endpoint.authorize(&mut req).await;
        
        match OAuthResponse::from(response).into_response() {
            Ok(resp) => resp,
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }

    // Token endpoint to exchange authorization code for an access token
    pub async fn token(
        &self,
        req: HttpRequest,
        body: web::Form<std::collections::HashMap<String, String>>,
    ) -> impl Responder {
        let mut req = OAuthRequest::from_actix(&req, body.into_inner());
        let response = self.auth_config.endpoint.access_token(&mut req).await;
        
        match OAuthResponse::from(response).into_response() {
            Ok(resp) => resp,
            Err(OAuthError::AccessDenied) => HttpResponse::Unauthorized().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
