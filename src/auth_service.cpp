#include "auth_service.h"
#include <iostream>
#include <openssl/sha.h>

AuthService::AuthService() {
    try {
        redisClient = std::make_shared<sw::redis::Redis>("tcp://127.0.0.1:6379");
    } catch (const sw::redis::Error &err) {
        std::cerr << "Failed to connect to Redis: " << err.what() << std::endl;
    }
}

AuthService::~AuthService() {}

bool AuthService::registerUser(const std::string &username, const std::string &password) {
    std::string hashedPassword = hashPassword(password);
    auto userKey = "user:" + username;

    try {
        auto userExists = redisClient->exists(userKey);
        if (userExists) {
            std::cerr << "User already exists!" << std::endl;
            return false;
        }

        redisClient->set(userKey, hashedPassword);
        return true;
    } catch (const sw::redis::Error &err) {
        std::cerr << "Failed to register user: " << err.what() << std::endl;
        return false;
    }
}

bool AuthService::authenticateUser(const std::string &username, const std::string &password) {
    auto userKey = "user:" + username;
    std::string hashedPassword = hashPassword(password);

    try {
        auto storedPassword = redisClient->get(userKey);
        if (!storedPassword) {
            std::cerr << "User not found!" << std::endl;
            return false;
        }

        return *storedPassword == hashedPassword;
    } catch (const sw::redis::Error &err) {
        std::cerr << "Failed to authenticate user: " << err.what() << std::endl;
        return false;
    }
}

std::string AuthService::hashPassword(const std::string &password) {
    unsigned char hash[SHA256_DIGEST_LENGTH];
    SHA256_CTX sha256;
    SHA256_Init(&sha256);
    SHA256_Update(&sha256, password.c_str(), password.size());
    SHA256_Final(hash, &sha256);

    std::stringstream ss;
    for (int i = 0; i < SHA256_DIGEST_LENGTH; i++) {
        ss << std::hex << (int)hash[i];
    }
    return ss.str();
}
