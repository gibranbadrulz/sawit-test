PROJECT_NAME := sawit-log
BUILD_DIR := target
SRC_DIR := src
TEST_DIR := tests

DEBUG ?= true

CARGO := cargo
CARGO_FLAGS :=

ifeq ($(DEBUG),true)
    CARGO_FLAGS += --verbose
else
    CARGO_FLAGS += --quiet
endif

.PHONY: all build release install clean test help

all: build

build:
	$(CARGO) build $(CARGO_FLAGS)

release:
	$(CARGO) build --release $(CARGO_FLAGS)

install:
	$(CARGO) install --path . $(CARGO_FLAGS)

clean:
	$(CARGO) clean $(CARGO_FLAGS)

test:
	$(CARGO) test $(CARGO_FLAGS)

run:
	$(CARGO) run $(CARGO_FLAGS)

help:
	@echo "Usage: make [TARGET]"
	@echo ""
	@echo "Targets:"
	@echo "  all      - Build the project (default)"
	@echo "  build    - Build the project"
	@echo "  release  - Build the project in release mode"
	@echo "  install  - Install the project"
	@echo "  clean    - Clean the project"
	@echo "  test     - Run tests"
	@echo "  run      - Run the project"
	@echo "  help     - Display this help message"
	@echo ""
	@echo "Options:"
	@echo "  DEBUG    - Set to 'true' for verbose output, 'false' for quiet (default: true)"
