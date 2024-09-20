# buildscripts/redis-plus-plus.mk
# This file handles the build logic for redis-plus-plus.

# Variables
REDIS_PLUS_PLUS_REPO = https://github.com/sewenew/redis-plus-plus.git

# Clone and build redis-plus-plus
redis-plus-plus: hiredis
	@if [ ! -d "redis-plus-plus" ]; then \
		echo "Cloning redis-plus-plus..."; \
		git clone $(REDIS_PLUS_PLUS_REPO); \
	fi
	cd redis-plus-plus && mkdir -p build && cd build; \
	cmake ..; \
	make; \
	sudo make install
