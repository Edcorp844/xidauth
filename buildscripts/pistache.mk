# buildscripts/pistache.mk
# This file handles the build logic for Pistache.

# Variables
PISTACHE_REPO = https://github.com/oktal/pistache.git

# Clone and build Pistache
pistache: googletest
	@if [ ! -d "pistache" ]; then \
		echo "Cloning Pistache..."; \
		git clone $(PISTACHE_REPO); \
	fi
	cd pistache && mkdir -p build && cd build; \
	if [ $(DISABLE_TESTS) -eq 1 ]; then \
		echo "Building Pistache with tests disabled..."; \
		cmake -DPISTACHE_BUILD_TESTS=OFF ..; \
	else \
		echo "Building Pistache with tests enabled..."; \
		cmake ..; \
	fi; \
	make; \
	sudo make install
