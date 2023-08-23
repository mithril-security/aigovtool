#!/bin/bash


# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Changing default to nightly
rustup default nightly
rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly
source "$HOME/.cargo/env"

# Fortanix & Just
cargo install fortanix-sgx-tools ftxsgx-simulator sgxs-tools --git https://github.com/mithril-security/rust-sgx --branch sim-mode

export BLINDAI_AZURE_DCSV3_PATCH=1
export SGX_AESM_ADDR=1

# poetry installation
curl -sSL https://install.python-poetry.org | python3 -

git submodule init
git submodule update




