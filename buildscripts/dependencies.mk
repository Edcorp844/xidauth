# buildscripts/dependencies.mk
# This file handles the installation of dependencies like GoogleTest and Hiredis.

# Variables
GOOGLETEST_REPO = https://github.com/google/googletest.git
HIREDIS_REPO = https://github.com/redis/hiredis.git

# Target for installing GoogleTest and GoogleMock if missing
googletest:
	@if [ ! -d "/usr/local/include/gtest" ]; then \
		echo "GoogleTest/GoogleMock not found. Installing..."; \
		git clone $(GOOGLETEST_REPO); \
		cd googletest; \
		mkdir build && cd build; \
		cmake ..; \
		make; \
		sudo make install; \
	else \
		echo "GoogleTest/GoogleMock already installed."; \
	fi

# Target for installing Hiredis
hiredis:
	@if [ ! -d "/usr/local/include/hiredis" ]; then \
		echo "Hiredis not found. Installing..."; \
		git clone $(HIREDIS_REPO); \
		cd hiredis; \
		make; \
		sudo make install; \
	else \
		echo "Hiredis already installed."; \
	fi
