#ifndef AUTH_SERVICE_H
#define AUTH_SERVICE_H

#include <sw/redis++/redis++.h>
#include <string>

class AuthService {
public:
    AuthService();
    ~AuthService();

    bool registerUser(const std::string& username, const std::string& password);
    bool authenticateUser(const std::string& username, const std::string& password);

private:
    std::shared_ptr<sw::redis::Redis> redisClient;
    std::string hashPassword(const std::string& password);
};

#endif // AUTH_SERVICE_H
