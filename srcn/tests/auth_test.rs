#[cfg(test)]
mod tests {
    use crate::{auth::auth_service::AuthService, db::database::DataBase};

    use std::collections::HashMap;

    // Helper function to set up a test Redis connection
    fn setup_test_redis() -> DataBase {
        let redis_url = "redis://127.0.0.1:6379";
        DataBase::new(redis_url).expect("Failed to connect to Redis")
    }

    #[test]
    fn test_register_user() {
        let mut db = setup_test_redis();
        let mut auth_service =
            AuthService::new("redis://127.0.0.1/").expect("Failed to create AuthService");

        let user_id = "user_1";
        let mut user_info = HashMap::new();
        user_info.insert("username", "testuser");
        user_info.insert("password", "password123");

        let result = auth_service.register_user(user_id, &user_info);
        assert!(result.is_ok());

        // Verify the user is registered in Redis
        let json_data: Option<HashMap<String, String>> =
            Some(db.get_json_value(user_id).expect("Failed to get user data"));
        assert!(json_data.is_some());
    }

    #[test]
    fn test_authenticate_user_success() {
        let _db = setup_test_redis();
        let mut auth_service =
            AuthService::new("redis://127.0.0.1/").expect("Failed to create AuthService");

        let user_id = "user_2";
        let mut user_info = HashMap::new();
        user_info.insert("username", "testuser2");
        user_info.insert("password", "password123");

        // Register the user first
        auth_service
            .register_user(user_id, &user_info)
            .expect("Failed to register user");

        // Authenticate the user
        let is_authenticated = auth_service
            .authenticate_user(user_id, "password123")
            .expect("Authentication failed");
        assert!(is_authenticated);
    }

    #[test]
    fn test_authenticate_user_failure() {
        let _db = setup_test_redis();
        let mut auth_service =
            AuthService::new("redis://127.0.0.1/").expect("Failed to create AuthService");

        let user_id = "user_3";
        let mut user_info = HashMap::new();
        user_info.insert("username", "testuser3");
        user_info.insert("password", "password123");

        // Register the user first
        auth_service
            .register_user(user_id, &user_info)
            .expect("Failed to register user");

        // Attempt authentication with incorrect password
        let is_authenticated = auth_service
            .authenticate_user(user_id, "wrongpassword")
            .expect("Authentication failed");
        assert!(!is_authenticated);
    }

    #[test]
    fn test_store_and_get_oauth_token() {
        let  _db = setup_test_redis();
        let mut auth_service =
            AuthService::new("redis://127.0.0.1/").expect("Failed to create AuthService");

        let user_id = "user_4";
        let token = "sample_oauth_token";

        // Store the token
        auth_service
            .store_oauth_token(user_id, token, 60)
            .expect("Failed to store OAuth token");

        // Retrieve the token
        let retrieved_token = auth_service
            .get_oauth_token(user_id)
            .expect("Failed to get OAuth token");
        assert_eq!(retrieved_token, Some(token.to_string()));
    }

    #[test]
    fn test_set_and_validate_session() {
        let  _db = setup_test_redis();
        let mut auth_service =
            AuthService::new("redis://127.0.0.1/").expect("Failed to create AuthService");

        let session_id = "session_1";
        let user_id = "user_5";

        // Set the session
        auth_service
            .set_session(session_id, user_id, 60)
            .expect("Failed to set session");

        // Validate the session
        let validated_user_id = auth_service
            .validate_session(session_id)
            .expect("Failed to validate session");
        assert_eq!(validated_user_id, Some(user_id.to_string()));
    }

    #[test]
    fn test_delete_session() {
        let _db = setup_test_redis();
        let mut auth_service =
            AuthService::new("redis://127.0.0.1/").expect("Failed to create AuthService");

        let session_id = "session_2";
        let user_id = "user_6";

        // Set the session
        auth_service
            .set_session(session_id, user_id, 60)
            .expect("Failed to set session");

        // Delete the session
        auth_service
            .delete_session(session_id)
            .expect("Failed to delete session");

        // Validate the session should return None
        let validated_user_id = auth_service
            .validate_session(session_id)
            .expect("Failed to validate session");
        assert_eq!(validated_user_id, None);
    }
}
