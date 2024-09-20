# Main Makefile

# Compiler settings
CXX = g++
CXXFLAGS = -std=c++17 -Wall -Iinclude -I/usr/local/include -I/usr/local/include/sw -I/usr/local/include/pistache
LDFLAGS = -L/usr/local/lib -lssl -lcrypto -lpthread -lredis++ -lpistache

# Source files and object files
SRCDIR = src
OBJDIR = obj
BINDIR = bin
SOURCES = $(wildcard $(SRCDIR)/*.cpp)
OBJECTS = $(patsubst $(SRCDIR)/%.cpp, $(OBJDIR)/%.o, $(SOURCES))
TARGET = $(BINDIR)/auth_microservice

# Include other build scripts
include buildscripts/dependencies.mk
include buildscripts/pistache.mk
include buildscripts/redis-plus-plus.mk

# Default target
all: check-pistache check-redis $(TARGET)

# Link object files to create the final executable
$(TARGET): $(OBJECTS)
	$(CXX) $(OBJECTS) -o $@ $(LDFLAGS)

# Compile each source file into an object file
$(OBJDIR)/%.o: $(SRCDIR)/%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean up build artifacts
clean:
	sudo rm -rf $(OBJDIR)/*.o $(BINDIR)/auth_microservice
	sudo rm -rf pistache redis-plus-plus googletest hiredis

# Clean dependencies
clean-dependencies: clean
	rm -rf pistache/build redis-plus-plus/build googletest/build hiredis/build

