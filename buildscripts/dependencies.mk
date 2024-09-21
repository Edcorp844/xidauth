# Path to the local dependencies folder
DEPS_DIR := $(PWD)/deps

# Redis-plus-plus installation
REDIS_PLUS_PLUS_DIR := $(DEPS_DIR)/redis-plus-plus
HIREDIS_DIR := $(DEPS_DIR)/hiredis
OPENSSL_DIR := $(DEPS_DIR)/openssl


# Target to download and install dependencies
.PHONY: deps
deps: hiredis redis-plus-plus openssl

hiredis:
	@if [ ! -d $(HIREDIS_DIR) ]; then \
		git clone https://github.com/redis/hiredis.git $(HIREDIS_DIR); \
		cd $(HIREDIS_DIR) && make && make install PREFIX=$(DEPS_DIR); \
	fi

redis-plus-plus:
	@if [ ! -d $(REDIS_PLUS_PLUS_DIR) ]; then \
		git clone https://github.com/sewenew/redis-plus-plus.git $(REDIS_PLUS_PLUS_DIR); \
		cd $(REDIS_PLUS_PLUS_DIR) && mkdir -p build && cd build && \
		cmake -DCMAKE_INSTALL_PREFIX=$(DEPS_DIR) .. && make && make install; \
	fi

openssl:
	@if [ ! -d $(OPENSSL_DIR) ]; then \
		curl -o openssl.tar.gz https://www.openssl.org/source/openssl-1.1.1k.tar.gz; \
		tar -xzf openssl.tar.gz -C $(DEPS_DIR); \
		mv $(DEPS_DIR)/openssl-1.1.1k $(OPENSSL_DIR); \
		cd $(OPENSSL_DIR) && ./config --prefix=$(OPENSSL_DIR) --openssldir=$(OPENSSL_DIR) && make && make install; \
	fi
