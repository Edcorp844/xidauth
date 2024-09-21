CXX = g++
CXXFLAGS = -std=c++17 -Iinclude -I$(DEPS_DIR)/include -I$(DEPS_DIR)/redis-plus-plus/include -I$(DEPS_DIR)/openssl/include -pthread
LDFLAGS = -L$(DEPS_DIR)/lib -L$(DEPS_DIR)/openssl/lib -lhiredis -lredis++ -lssl -lcrypto

SRCS = $(wildcard src/*.cpp)   # Automatically includes all .cpp files in src/
OBJS = $(SRCS:.cpp=.o)
TARGET = authid                # Executable name is now authid

.PHONY: all clean

include buildscripts/dependencies.mk

all: $(TARGET)

# Target to compile the executable
$(TARGET): $(OBJS)
	$(CXX) -o $@ $^ $(CXXFLAGS) $(LDFLAGS)

# Clean target to remove object files and the executable
clean:
	rm -f $(OBJS) $(TARGET)
	rm -rf $(DEPS_DIR)
