# buildscripts/dependencies.mk
# This file handles the installation of dependencies like 
# GoogleTest, Hiredis, Redis-plus-plus, and Pistache

# Variables
GOOGLETEST_REPO = https://github.com/google/googletest.git
HIREDIS_REPO = https://github.com/redis/hiredis.git
PISTACHE_REPO = https://github.com/oktal/pistache.git
REDIS_PLUS_PLUS_REPO = https://github.com/sewenew/redis-plus-plus.git

# Master target to install all dependencies
all-dependencies: redis-plus-plus pistache

# Clone and build redis-plus-plus
redis-plus-plus: hiredis
	@if [ ! -d "dependencies/redis-plus-plus" ]; then \
		echo "Cloning redis-plus-plus..."; \
		git clone $(REDIS_PLUS_PLUS_REPO) dependencies/redis-plus-plus; \
	fi
	cd dependencies/redis-plus-plus && mkdir -p build && cd build; \
	cmake -DCMAKE_PREFIX_PATH=../../lib/hiredis -DCMAKE_INSTALL_PREFIX=../../lib/redis-plus-plus ..; \
	make; \
	make install

# Clone and build Pistache
pistache: #googletest
	@if [ ! -d "dependencies/pistache" ]; then \
		echo "Cloning Pistache..."; \
		git clone $(PISTACHE_REPO) dependencies/pistache; \
	fi
	cd dependencies/pistache && \
	meson build --buildtype=release; \
	cd build && \
	ninja && \
	ninja install

# Target for installing GoogleTest and GoogleMock if missing
googletest:
	@if [ ! -d "/usr/local/include/gtest" ]; then \
		echo "GoogleTest/GoogleMock not found. Installing..."; \
		git clone $(GOOGLETEST_REPO) libs/googletest; \
		cd libs/googletest; \
		mkdir build && cd build; \
		cmake ..; \
		make; \
		sudo make install; \
	else \
		echo "GoogleTest/GoogleMock already installed."; \
	fi

# Target for installing Hiredis
hiredis:
	@if [ ! -d "lib/hiredis" ]; then \
		echo "Hiredis not found. Installing..."; \
		git clone $(HIREDIS_REPO) dependencies/hiredis; \
		cd dependencies/hiredis; \
		make; \
		make PREFIX=../../lib/hiredis  install; \
	else \
		echo "Hiredis already installed."; \
	fi
