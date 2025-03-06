#[cfg(test)]
mod tests {

    use uuid::Uuid;
    use crate::auth::auth_service::AuthService;
    use crate::database::database::Database;
    use crate::database::database_error::DatabaseError;
    use crate::database::utils::BlacklistedToken;

    // Helper function to set up AuthService with a mock database
    fn setup_auth_service() -> AuthService {
        let redis_url = "redis://127.0.0.1/"; // Mock Redis URL
        let encryption_key: [u8; 32] = [0; 32]; // Mock encryption key

        let database = Database::new(redis_url, &encryption_key).unwrap();
        AuthService::new(database)
    }

    #[test]
    fn test_register_user_success() {
        let auth_service = setup_auth_service();
        let email = "test@example.com".to_string();
        let password = "securepassword".to_string();

        let result = auth_service.register_user(email.clone(), password.clone());

        // Assert that the registration is successful
        assert!(result.is_ok());
        assert!(result.unwrap()); // User should be registered
    }

    #[test]
    fn test_register_user_duplicate() {
        let auth_service = setup_auth_service();
        let email = "duplicate@example.com".to_string();
        let password = "password".to_string();

        // First registration attempt should succeed
        auth_service
            .register_user(email.clone(), password.clone())
            .unwrap();

        // Second attempt with the same email should return an error
        let second_attempt = auth_service.register_user(email, password);

        // Assert that the second attempt returns an error (DatabaseError::UserAlreadyExists)
        assert!(second_attempt.is_err());
        match second_attempt {
            Err(DatabaseError::UserAlreadyExists) => {} // Test passes if error is UserAlreadyExists
            _ => panic!("Expected UserAlreadyExists error"),
        }
    }

    #[test]
    fn test_login_user_success() {
        let auth_service = setup_auth_service();
        let email = "login@example.com".to_string();
        let password = "password123".to_string();

        // Register user first
        auth_service
            .register_user(email.clone(), password.clone())
            .unwrap();

        // Attempt to login with correct credentials
        let result = auth_service.login_user(email, password);

        // Assert that login is successful
        assert!(result.is_ok());
        assert!(result.unwrap()); // Login should succeed
    }

    #[test]
    fn test_login_user_invalid_credentials() {
        let auth_service = setup_auth_service();
        let email = "invalid@example.com".to_string();
        let password = "wrongpassword".to_string();

        // Attempt to login with invalid credentials
        let result = auth_service.login_user(email, password);

        // Assert that the result is an error
        assert!(result.is_err());

        // Check that the error is the expected InvalidCredentials error
        if let Err(DatabaseError::InvalidCredentials) = result {
            // Test passes
        } else {
            panic!("Expected InvalidCredentials error");
        }
    }
/* 
    #[test]
    fn test_blacklist_token() {
        let auth_service = setup_auth_service();
        let token = Uuid::new_v4().to_string();
        let reason = "Just For test".to_string();

        // Create a blacklisted token with expiration time
        let user = BlacklistedToken {
            token: token.clone(),
            expires_at: chrono::Utc::now().timestamp() + 3600, // Expires in 1 hour
            reason,
        };

        // Add the token to the blacklist
        let result = auth_service.add_blacklisted_token(user);
        assert!(result.is_ok());

        // Check if the token is blacklisted
        let check_blacklist = auth_service.is_token_blacklisted(&token);
        assert!(check_blacklist.is_ok());
        assert!(check_blacklist.unwrap()); // Token should be blacklisted
    } */
} 