docker stop redis-stack
docker rm redis-stack
docker run -d --name redis-stack -p 6379:6379 -p 8001:8001 redis/redis-stack:latest