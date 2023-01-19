SHELL = /usr/bin/env bash -o pipefail
.SHELLFLAGS = -ec

DOCKERUSERNAME = ${VAR_DOCKERHUB_USER}
DOCKERPASSWORD = ${VAR_DOCKERHUB_SECRET}
TOOLS_DIR := hack/tools
TOOLS_BIN_DIR := $(TOOLS_DIR)/bin
CARGO_DIR := $(HOME)/.cargo
CARGO_BINS := $(CARGO_DIR)/bin

# Set build time variables including version details
LDFLAGS := $(shell source ./hack/scripts/version.sh; version::ldflags)

PATH := $(abspath $(TOOLS_BIN_DIR)):$(PATH)

#$(TOOLS_BIN_DIR):
#	mkdir -p $@

##@ Linting

.PHONY: rustlint
rustlint: $(CARGO_BINS) ## Lint
	$(CARGO_BINS)/cargo clippy

.PHONY: clean
clean: $(CARGO_BINS) ## clean
	$(CARGO_BINS)/cargo clean

.PHONY: build
build: $(CARGO_BINS) ## build
	$(CARGO_BINS)/cargo build
