/* use crate::auth::auth_service::AuthService;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Mutex};

// Struct to hold user credentials for registration and authentication
#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

// Struct to hold user profile information
#[derive(Deserialize)]
pub struct Profile {
    first_name: String,
    last_name: String,
    profile_pic: String,
    gender: String,
    age: String,
    date_of_birth: String,
}

// Struct to hold the user data including credentials and profile
#[derive(Deserialize)]
pub struct UserData {
    credentials: Credentials,
    profile: Profile,
}

// Struct to hold the registration data
#[derive(Deserialize)]
pub struct RegistrationData {
    user: UserData,
}

// Standard response for successful operations
#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

// Standard response for error scenarios
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub error_code: String,
    pub message: String,
}

// Handler for user registration
#[post("/register")]
async fn register(
    data: web::Json<RegistrationData>, // JSON data containing registration info
    auth_service: web::Data<Mutex<AuthService>>, // Injecting the AuthService
) -> impl Responder {
    let email = &data.user.credentials.email; // Extracting email from the registration data

    // Lock the AuthService to ensure thread-safe access
    let mut auth_service = auth_service.lock().unwrap();
    // Check if the email already exists in the system
    if auth_service.check_email(email).unwrap_or(false) {
        // Email already exists, return a conflict response
        let response = ErrorResponse {
            status: "error".to_string(),
            error_code: "AUTH_EMAIL_EXISTS".to_string(),
            message: "Email already exists".to_string(),
        };
        return HttpResponse::Conflict().json(response);
    }

    // Prepare user information for registration
    let mut user_info = HashMap::new();
    user_info.insert("email", email.as_str());
    user_info.insert("password", data.user.credentials.password.as_str());
    user_info.insert("first_name", data.user.profile.first_name.as_str());
    user_info.insert("last_name", data.user.profile.last_name.as_str());
    user_info.insert("profile_pic", data.user.profile.profile_pic.as_str());
    user_info.insert("gender", data.user.profile.gender.as_str());
    user_info.insert("age", data.user.profile.age.as_str());
    user_info.insert("date_of_birth", data.user.profile.date_of_birth.as_str());

    // Attempt to register the user using AuthService
    match auth_service.register_user(email, &user_info) {
        Ok(_) => {
            // Registration successful, return a success response
            let response = SuccessResponse {
                status: "success".to_string(),
                message: "Registration successful".to_string(),
                data: json!({ "email": email }), // Returning the registered email
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            // Log the error and return an internal server error response
            let response = ErrorResponse {
                status: "error".to_string(),
                error_code: "AUTH_REGISTRATION_FAILED".to_string(),
                message: "Failed to register user".to_string(),
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Handler for user authentication
#[post("/authenticate")]
async fn authenticate(
    credentials: web::Json<Credentials>, // JSON data containing user credentials
    auth_service: web::Data<Mutex<AuthService>>, // Injecting the AuthService
) -> impl Responder {
    let email = &credentials.email; // Extracting email from credentials
    let password = &credentials.password; // Extracting password from credentials

    // Lock the AuthService for thread-safe access
    let mut auth_service = auth_service.lock().unwrap();

    // Check if the email exists in the system
    if auth_service.check_email(email).unwrap_or(false) {
        // Attempt to authenticate the user
        match auth_service.authenticate_user(email, password) {
            Ok(is_authenticated) => {
                if is_authenticated {
                    // Authentication successful, return a success response
                    let response = SuccessResponse {
                        status: "success".to_string(),
                        message: "Authentication successful".to_string(),
                        data: json!({ "email": email }), // You can return additional user info or tokens here
                    };
                    return HttpResponse::Ok().json(response);
                } else {
                    // Authentication failed, return an unauthorized response
                    let response = ErrorResponse {
                        status: "error".to_string(),
                        error_code: "AUTH_INVALID_CREDENTIALS".to_string(),
                        message: "Invalid email or password".to_string(),
                    };
                    return HttpResponse::Unauthorized().json(response);
                }
            }
            Err(_) => {
                // Log the error and return an internal server error response
                let response = ErrorResponse {
                    status: "error".to_string(),
                    error_code: "AUTH_AUTHENTICATION_FAILED".to_string(),
                    message: "Failed to authenticate user".to_string(),
                };
                return HttpResponse::InternalServerError().json(response);
            }
        }
    }

    // If the email does not exist, return a not found response
    let response = ErrorResponse {
        status: "error".to_string(),
        error_code: "AUTH_USER_NOT_FOUND".to_string(),
        message: "No user associated with the provided email".to_string(),
    };
    HttpResponse::NotFound().json(response)
}
 */

// Import necessary modules
use crate::auth::auth_service::AuthService;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Mutex};

// Struct to hold user credentials for registration and authentication
#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

// Struct to hold user profile information
#[derive(Deserialize)]
pub struct Profile {
    first_name: String,
    last_name: String,
    profile_pic: String,
    gender: String,
    age: String,
    date_of_birth: String,
}

// Struct to hold the user data including credentials and profile
#[derive(Deserialize)]
pub struct UserData {
    credentials: Credentials,
    profile: Profile,
}

// Struct to hold the registration data
#[derive(Deserialize)]
pub struct RegistrationData {
    user: UserData,
}

// Standard response for successful operations
#[derive(Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

// Standard response for error scenarios
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub error_code: String,
    pub message: String,
}

// Handler for user registration
#[post("/register")]
async fn register(
    data: web::Json<RegistrationData>, // JSON data containing registration info
    auth_service: web::Data<Mutex<AuthService>>, // Injecting the AuthService
) -> impl Responder {
    let email = &data.user.credentials.email; // Extracting email from the registration data

    // Lock the AuthService to ensure thread-safe access
    let mut auth_service = auth_service.lock().unwrap();

    // Check if email already exists
    if auth_service.check_email(email).unwrap_or(false) {
        return HttpResponse::Conflict().json(ErrorResponse {
            status: "error".to_string(),
            error_code: "AUTH_EMAIL_EXISTS".to_string(),
            message: "Email already exists".to_string(),
        });
    }

    // Prepare user information for registration
    let mut user_info = HashMap::new();
    user_info.insert("email", email.as_str());
    user_info.insert("password", data.user.credentials.password.as_str());
    user_info.insert("first_name", data.user.profile.first_name.as_str());
    user_info.insert("last_name", data.user.profile.last_name.as_str());
    user_info.insert("profile_pic", data.user.profile.profile_pic.as_str());
    user_info.insert("gender", data.user.profile.gender.as_str());
    user_info.insert("age", data.user.profile.age.as_str());
    user_info.insert("date_of_birth", data.user.profile.date_of_birth.as_str());

    // Attempt to register the user
    match auth_service.register_user(email, &user_info) {
        Ok(_) => HttpResponse::Ok().json(SuccessResponse {
            status: "success".to_string(),
            message: "User registered successfully".to_string(),
            data: json!({}),
        }),
        Err(_) => HttpResponse::InternalServerError().json(ErrorResponse {
            status: "error".to_string(),
            error_code: "AUTH_REGISTRATION_FAILED".to_string(),
            message: "Failed to register user".to_string(),
        }),
    }
}

// Handler for user authentication
#[post("/authenticate")]
async fn authenticate(
    data: web::Json<Credentials>, // JSON data containing user credentials
    auth_service: web::Data<Mutex<AuthService>>, // Injecting the AuthService
) -> impl Responder {
    let email = &data.email; // Extracting email from the credentials

    // Lock the AuthService to ensure thread-safe access
    let mut auth_service = auth_service.lock().unwrap();

    // Attempt to authenticate the user
    match auth_service.authenticate_user(email, &data.password) {
        Ok(true) => HttpResponse::Ok().json(SuccessResponse {
            status: "success".to_string(),
            message: "User authenticated successfully".to_string(),
            data: json!({}),
        }),
        Ok(false) => HttpResponse::Unauthorized().json(ErrorResponse {
            status: "error".to_string(),
            error_code: "AUTH_INVALID_CREDENTIALS".to_string(),
            message: "Invalid email or password".to_string(),
        }),
        Err(_) => HttpResponse::NotFound().json(ErrorResponse {
            status: "error".to_string(),
            error_code: "AUTH_USER_NOT_FOUND".to_string(),
            message: "User not found".to_string(),
        }),
    }
}
