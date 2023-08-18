#!/bin/bash

echo '>----------------------------------------------------------'
echo 'Intel SGX set up dependencies'
# download aesm for ubuntu
echo "deb https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -cs) main" | sudo tee -a /etc/apt/sources.list.d/intel-sgx.list >/dev/null
# add to apt-key list to authenticate package
curl -sSL "https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key" | sudo apt-key add -
# update available packages
sudo apt-get update \
# install aesm package
sudo apt-get install -y sgx-aesm-service libsgx-aesm-launch-plugin

sudo -u azureuser --  usermod -a -G aesmd $USER
sudo -u azureuser --  usermod -a -G sgx_prv aesmd
sudo -u azureuser --  usermod -a -G sgx_prv $USER

git submodule init
git submodule update

# setting rust and fortanix and dependencies
echo '>----------------------------------------------------------'
echo 'Setting rust and fortanix and dependencies'
sudo -u azureuser -- export CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup

sudo -u azureuser -- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo -u azureuser -- source "/usr/local/cargo/env"

sudo -u azureuser -- rustup default nightly \
    && sudo -u azureuser -- rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly \
    && sudo -u azureuser -- chmod -R g+r+w "${RUSTUP_HOME}"
apt-get update \
    && apt-get install -y pkg-config libssl-dev protobuf-compiler build-essential jq

sudo -u azureuser -- cargo install fortanix-sgx-tools ftxsgx-simulator sgxs-tools --git https://github.com/mithril-security/rust-sgx --branch sim-mode \
    && sudo -u azureuser -- chmod -R g+r+w "${CARGO_HOME}"

# Add just
sudo -u azureuser -- curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | sudo -u azureuser --  bash -s -- --to /usr/bin

# Azure DCAP remote  attestation dependencies
echo '>----------------------------------------------------------'
echo 'Azure DCAP remote  attestation dependencies'
 # Install temp dependencies
TEMP_DEPENDENCIES="curl gnupg software-properties-common" && \
apt-get update -y && apt-get install -y $TEMP_DEPENDENCIES && \

# Removing the default quote providing library in order to avoid conflicts
apt-get remove -y libsgx-dcap-default-qpl && \

# Install azure_dcap_client
curl -sSL https://packages.microsoft.com/keys/microsoft.asc | apt-key add - && \
add-apt-repository "https://packages.microsoft.com/ubuntu/20.04/prod" && \
apt-get update && apt-get install -y az-dcap-client && \
ln -s /usr/lib/libdcap_quoteprov.so /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so.1 && \

# Remove temp dependencies
apt-get remove -y $TEMP_DEPENDENCIES && apt-get autoremove -y && \
rm -rf /var/lib/apt/lists/* && rm -rf /var/cache/apt/archives/*

export BLINDAI_AZURE_DCSV3_PATCH=1
apt-get update -y 
apt-get install -y \
    libcurl4 \
    libssl1.1 \
    make \
    cmake \
    libpython3.8-dev

# # Add poetry and python dev utilities
# echo '>----------------------------------------------------------'
# echo 'Add poetry and python dev utilities'
# su azureuser apt-get install -y python3-dev python3-distutils python3-pip \
# && curl -sSL https://install.python-poetry.org | python3 -
# su azureuser export PATH=$HOME/.local/bin
