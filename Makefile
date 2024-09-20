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

.PHONY: all $(TARGET) $(OBJDIR)/%.o clean clean-dependencies

# Default target
all: all-dependencies $(TARGET)

# Link object files to create the final executable clean-dependencies
$(TARGET): $(OBJECTS)
	$(CXX) $(OBJECTS) -o $@ $(LDFLAGS)

# Compile each source file into an object file
$(OBJDIR)/%.o: $(SRCDIR)/%.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean up build artifacts
clean:
	rm -rf $(OBJDIR)/*.o $(BINDIR)/auth_microservice
	

# Clean dependencies
clean-dependencies: 
	rm -rf dependencies
	
