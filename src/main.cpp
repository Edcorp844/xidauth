#include "auth_service.h"
#include <iostream>

int main(int argc,char ** argv) {
    AuthService authService;

    // Register a user
    if (authService.registerUser("user1", "password123")) {
        std::cout << "User registered successfully." << std::endl;
    }

    // Authenticate the user
    if (authService.authenticateUser("user1", "password123")) {
        std::cout << "Authentication successful." << std::endl;
    } else {
        std::cout << "Authentication failed." << std::endl;
    }

    return EXIT_SUCCESS;
}
