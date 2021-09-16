SHELL                      := /bin/bash

IPFS_API_PATH              := ipfs-api

CARGO_BIN                  := $(shell which cargo)

WORKSPACE_CARGO_FILE       := Cargo.toml

README.md: README.tpl $(WORKSPACE_CARGO_FILE) $(IPFS_API_PATH)/Cargo.toml $(IPFS_API_PATH)/src/lib.rs
	$(CARGO_BIN) readme -r $(IPFS_API_PATH) -t ../README.tpl -o ../$@
