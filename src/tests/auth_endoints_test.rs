/* #[cfg(test)]
mod tests {
    use crate::auth::{auth_error::AuthError, auth_service::*, endpoints::*};
    use crate::db::{database::*, db_error::*};
    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // Define the AuthService trait
    trait AuthService {
        fn check_email(&mut self, email: &str) -> Result<bool, DBError>;
        fn register_user(&mut self, email: &str, user_info: &HashMap<&str, &str>) -> Result<(), AuthError>;
        fn authenticate_user(&mut self, email: &str, password: &str) -> Result<bool, DBError>;
    }

    // Mock AuthService
    struct MockAuthService {
        should_exist: bool,
        should_authenticate: bool,
    }

    impl MockAuthService {
        fn new(should_exist: bool, should_authenticate: bool) -> Self {
            Self {
                should_exist,
                should_authenticate,
            }
        }
    }

    impl AuthService for MockAuthService {
        fn check_email(&mut self, _email: &str) -> Result<bool, DBError> {
            Ok(self.should_exist)
        }

        fn register_user(&mut self, _email: &str, _user_info: &HashMap<&str, &str>) -> Result<(), AuthError> {
            if self.should_exist {
                Err(AuthError::UserAlreadyExists)
            } else {
                Ok(())
            }
        }

        fn authenticate_user(&mut self, _email: &str, _password: &str) -> Result<bool, DBError> {
            Ok(self.should_authenticate)
        }
    }

    #[actix_web::test]
    async fn test_register_success() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(false, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(register),
        )
        .await;

        let registration_data = json!({
            "user": {
                "credentials": { "email": "test@example.com", "password": "password123" },
                "profile": { "first_name": "John", "last_name": "Doe", "profile_pic": "url", "gender": "male", "age": "30", "date_of_birth": "1994-01-01" }
            }
        });

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&registration_data)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: SuccessResponse<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(body.status, "success");
    }

    #[actix_web::test]
    async fn test_register_email_exists() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(true, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(register),
        )
        .await;

        let registration_data = json!({
            "user": {
                "credentials": { "email": "test@example.com", "password": "password123" },
                "profile": { "first_name": "John", "last_name": "Doe", "profile_pic": "url", "gender": "male", "age": "30", "date_of_birth": "1994-01-01" }
            }
        });

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&registration_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CONFLICT);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "error");
        assert_eq!(body.error_code, "AUTH_EMAIL_EXISTS");
    }

    #[actix_web::test]
    async fn test_authenticate_success() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(true, true)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(authenticate),
        )
        .await;

        let credentials = json!({ "email": "test@example.com", "password": "password123" });
        
        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: SuccessResponse<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(body.status, "success");
    }

    #[actix_web::test]
    async fn test_authenticate_invalid_credentials() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(true, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(authenticate),
        )
        .await;

        let credentials = json!({ "email": "test@example.com", "password": "wrong_password" });

        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "error");
        assert_eq!(body.error_code, "AUTH_INVALID_CREDENTIALS");
    }

    #[actix_web::test]
    async fn test_authenticate_user_not_found() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(false, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(authenticate),
        )
        .await;

        let credentials = json!({ "email": "nonexistent@example.com", "password": "password123" });

        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "error");
        assert_eq!(body.error_code, "AUTH_USER_NOT_FOUND");
    }
}
 */

 

 #[cfg(test)]
mod tests {
    use crate::auth::{auth_error::AuthError, auth_service::*, endpoints::*};
    use crate::db::{database::*, db_error::*};
    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // Define the AuthService trait
    trait AuthService {
        fn check_email(&mut self, email: &str) -> Result<bool, DBError>;
        fn register_user(&mut self, email: &str, user_info: &HashMap<&str, &str>) -> Result<(), AuthError>;
        fn authenticate_user(&mut self, email: &str, password: &str) -> Result<bool, DBError>;
    }

    // Mock AuthService
    struct MockAuthService {
        should_exist: bool,
        should_authenticate: bool,
    }

    impl MockAuthService {
        fn new(should_exist: bool, should_authenticate: bool) -> Self {
            Self {
                should_exist,
                should_authenticate,
            }
        }
    }

    impl AuthService for MockAuthService {
        fn check_email(&mut self, _email: &str) -> Result<bool, DBError> {
            Ok(self.should_exist)
        }

        fn register_user(&mut self, _email: &str, _user_info: &HashMap<&str, &str>) -> Result<(), AuthError> {
            if self.should_exist {
                Err(AuthError::UserAlreadyExists)
            } else {
                Ok(())
            }
        }

        fn authenticate_user(&mut self, _email: &str, _password: &str) -> Result<bool, DBError> {
            Ok(self.should_authenticate)
        }
    }

    #[actix_web::test]
    async fn test_register_success() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(false, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(register),
        )
        .await;

        let registration_data = json!({
            "user": {
                "credentials": { "email": "test@example.com", "password": "password123" },
                "profile": { "first_name": "John", "last_name": "Doe", "profile_pic": "url", "gender": "male", "age": "30", "date_of_birth": "1994-01-01" }
            }
        });

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&registration_data)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: SuccessResponse<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(body.status, "success");
    }

    #[actix_web::test]
    async fn test_register_email_exists() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(true, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(register),
        )
        .await;

        let registration_data = json!({
            "user": {
                "credentials": { "email": "test@example.com", "password": "password123" },
                "profile": { "first_name": "John", "last_name": "Doe", "profile_pic": "url", "gender": "male", "age": "30", "date_of_birth": "1994-01-01" }
            }
        });

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(&registration_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CONFLICT);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "error");
        assert_eq!(body.error_code, "AUTH_EMAIL_EXISTS");
    }

    #[actix_web::test]
    async fn test_authenticate_success() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(true, true)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(authenticate),
        )
        .await;

        let credentials = json!({ "email": "test@example.com", "password": "password123" });
        
        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: SuccessResponse<serde_json::Value> = test::read_body_json(resp).await;
        assert_eq!(body.status, "success");
    }

    #[actix_web::test]
    async fn test_authenticate_invalid_credentials() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(true, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(authenticate),
        )
        .await;

        let credentials = json!({ "email": "test@example.com", "password": "wrong_password" });

        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "error");
        assert_eq!(body.error_code, "AUTH_INVALID_CREDENTIALS");
    }

    #[actix_web::test]
    async fn test_authenticate_user_not_found() {
        let auth_service = Arc::new(Mutex::new(MockAuthService::new(false, false)));
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(auth_service.clone()))
                .service(authenticate),
        )
        .await;

        let credentials = json!({ "email": "nonexistent@example.com", "password": "password123" });

        let req = test::TestRequest::post()
            .uri("/authenticate")
            .set_json(&credentials)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body: ErrorResponse = test::read_body_json(resp).await;
        assert_eq!(body.status, "error");
        assert_eq!(body.error_code, "AUTH_USER_NOT_FOUND");
    }
}
