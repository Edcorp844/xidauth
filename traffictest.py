import threading
import requests
import random
import json
import time

# Server URL (update this if needed)
BASE_URL: str = "http://localhost:8080"  # Adjust the port as necessary

# Sample user data
sample_users: list[dict[str:str]] = [
    {
        "email": f"user{i}@example.com",
        "password": "password123",
        "first_name": f"FirstName{i}",
        "last_name": f"LastName{i}",
        "profile_pic": "http://example.com/profile_pic.png",
        "gender": random.choice(["male", "female"]),
        "age": str(random.randint(18, 65)),
        "date_of_birth": "2000-01-01",
    }
    for i in range(1000)
]


# Function to register a user
def register_user(user_data: dict) -> None:
    registration_data: dict[str:any] = {
        "user": {
            "credentials": {
                "email": user_data["email"],
                "password": user_data["password"],
            },
            "profile": {
                "first_name": user_data["first_name"],
                "last_name": user_data["last_name"],
                "profile_pic": user_data["profile_pic"],
                "gender": user_data["gender"],
                "age": user_data["age"],
                "date_of_birth": user_data["date_of_birth"],
            },
        }
    }
    response = requests.post(f"{BASE_URL}/register", json=registration_data)
    print(
        f"Register response for {user_data['email']}: {response.status_code} - {response.text}"
    )


# Function to authenticate a user
def authenticate_user(user_data) -> None:
    credentials: dict[str:any] = {
        "email": user_data["email"],
        "password": user_data["password"],
    }
    response: Response = requests.post(f"{BASE_URL}/authenticate", json=credentials)
    print(
        f"Authenticate response for {user_data['email']}: {response.status_code} - {response.text}"
    )


# Function to perform random requests
def random_request() -> None:
    user_data: dict[str:str] = random.choice(sample_users)  # Pick a random user
    if random.choice([True, False]):
        register_user(user_data)  # Randomly register a user
    else:
        authenticate_user(user_data)  # Randomly authenticate a user


# Create and start threads
threads: list = []
for _ in range(1000):
    thread: thread = threading.Thread(target=random_request)
    threads.append(thread)
    thread.start()

# Wait for all threads to complete
for thread in threads:
    thread.join()

print("All requests completed.")
