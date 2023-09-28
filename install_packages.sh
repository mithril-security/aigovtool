#!/bin/bash

set -e

apt-get install -y libcurl4 libssl1.1 make cmake jq pkg-config libssl-dev protobuf-compiler curl gnupg software-properties-common

# Intel SGX 
echo "deb https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -cs) main" | tee -a /etc/apt/sources.list.d/intel-sgx.list >/dev/null 

curl -sSL "https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key" | apt-key add - 
apt-get update
apt-get install -y sgx-aesm-service libsgx-aesm-launch-plugin
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/bin
apt-get remove -y libsgx-dcap-default-qpl 
curl -sSL https://packages.microsoft.com/keys/microsoft.asc | apt-key add -
sudo add-apt-repository "https://packages.microsoft.com/ubuntu/20.04/prod"
apt-get update && apt-get install -y az-dcap-client
ln -s /usr/lib/libdcap_quoteprov.so /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so.1
