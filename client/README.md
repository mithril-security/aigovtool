<a name="readme-top"></a>



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/mithril-security/BlindAI">
    <img src="https://github.com/mithril-security/BlindAI/raw/main/docs/assets/logo.png" alt="Logo" width="80" height="80">
  </a>

<h1 align="center">BlindAI DRM Proof-Of-Concept</h1>

[![Website][website-shield]][website-url]
[![Blog][blog-shield]][blog-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

  <p align="center">
    <b>BlindAI DRM Proof-Of-Concept</b> is an <b>AI privacy solution</b>, allowing AI model providers to 'lend' their AI models for on-premise hosting without exposing their model weights and while retaining full control over usage.	<br /><br />
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#-about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#-Set-up">Set up</a>
    </li>
    <li><a href="#-usage">Usage</a></li>
    <li><a href="#-getting-help">Getting Help</a></li>
    <li><a href="#-license">License</a></li>
    <li><a href="#-contact">Contact</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## 🔒 About The Project

**BlindAI DRM POC** is an Open-Source solution enabling AI providers to 'lend' their models for on-premise deployment while **guaranteeing privacy for both their model weights and end user's data**. It includes a DRM Server and client to monitor inference consumption of a particular model and grant or block access.


### Key actors
In order to understand BlindAI DRM, let’s first define the three key actors in our secure AI consumption process:

- **The custodian​:** Their role is to provide the AI model, track and potentially block AI consumption
- **The AI borrower**: The borrower deploys the custodian's AI model on their infrastructure. This actor may or may not also be the final end user.
- **The AI consumer​**: The AI consumer is the end user who queries the model​ hosted by the AI borrower.

### Key components

BlindAI DRM is made up of three main components:

#### The DRM custodian server

This is the server used by the custodian to:​
  - Securely share the model to the enclave used by the AI consumer​
  - Block or unlock model consumption for end users
  - Follow the consumption of AI models​

#### The enclave AI server

This server is used by AI consumers to locally host the AI model. This model weights are never directly accessible and remain encrypted in memory thanks to the use of secure enclaves.

### End user server
The client server for AI consumers to query the model inside the Enclave AI server.

### Built With 

[![Rust][Rust]][Rust-url] [![Python][Python]][Python-url] [![Intel-SGX][Intel-SGX]][Intel-sgx-url] [![Tract][Tract]][tract-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Enclave server set up

In order to run the enclave server, the AI borrower will need to set-up a compatible VM and install the required dependencies.

### Choosing the VM
There are different VMs available on Azure for running confidential computing applications. 

As we are working on intel SGX, we are going to choose the DCs v3 family which supports Intel SGX (and more precisely SGX 2). To have enough memory to run our models, we choose the 64gb memory with 8-vcpus.
![Azure VM](https://github.com/mithril-security/BlindAI_drm_fli/blob/main/docs/assets/set_up.png)

After the creation of the instance, we can connect to it via SSH. You can do this in `Visual Studio Code` with the `remote container VSCode extension`.

Once you have installed this extension, you can click on the green menu in the bottom-left of VSCode and select `connect to host` before supplying your host address, `azureuser@VM_IP_ADDRESS`.

![vscode_menu](https://raw.githubusercontent.com/mithril-security/BlindAI/main/docs/assets/Screenshot-vscode.png)

![vscode_connect](https://raw.githubusercontent.com/mithril-security/BlindAI/main/docs/assets/host.png)


### Setting up Intel SGX and the required dependencies 
After connecting to the instance via SSH you can run the following scripts to install SGX, rust, and all the configuration needed to run our BlindAI secure enclave. 

We begin by cloning the BlindAI DRM repo via github :
```bash
$ git clone https://github.com/mithril-security/BlindAI_drm_fli.git
$ cd blindai_drm_fli/
```

- The first script installs all the dependencies needed for SGX and remote attestation to work perfectly, this one should be ran as root : 
```bash 
$ sudo ./install_packages.sh
``` 

- Then, run the following script in normal user to finish the installation configuration.
```bash
$ ./install_config.sh
```

After this point, a reboot is necessary so that SGX works. You can directly reboot the instance from the Azure portal. 


<!-- - Usual dependencies
```bash 
sudo apt-get install -y libcurl4 libssl1.1 make cmake jq pkg-config libssl-dev protobuf-compiler curl gnupg software-properties-common
```

- Rust and its set-up
```bash
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Changing default to nightly
rustup default nightly
rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly
```

- Installing intel SGX dependencies and fortanix
```bash
# Intel SGX 
echo "deb https://download.01.org/intel-sgx/sgx_repo/ubuntu $(lsb_release -cs) main" | sudo tee -a /etc/apt/sources.list.d/intel-sgx.list >/dev/null 

curl -sSL "https://download.01.org/intel-sgx/sgx_repo/ubuntu/intel-sgx-deb.key" | sudo apt-key add - 

sudo apt-get update

sudo apt-get install -y sgx-aesm-service libsgx-aesm-launch-plugin

# Fortanix & Just
cargo install fortanix-sgx-tools ftxsgx-simulator sgxs-tools --git https://github.com/mithril-security/rust-sgx --branch sim-mode

curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | sudo bash -s -- --to /usr/bin

# Azure DCAP client
sudo apt-get remove -y libsgx-dcap-default-qpl 
curl -sSL https://packages.microsoft.com/keys/microsoft.asc | sudo apt-key add -
sudo add-apt-repository "https://packages.microsoft.com/ubuntu/20.04/prod"
sudo apt-get update && sudo apt-get install -y az-dcap-client
sudo ln -s /usr/lib/libdcap_quoteprov.so /usr/lib/x86_64-linux-gnu/libdcap_quoteprov.so.1
``` -->
At this point, everything related to Intel SGX has been installed.

_For more examples on the BlindAI project, you can refer to the [Documentation](https://BlindAI.mithrilsecurity.io/en/latest/)_

## Pre-requisite steps on Azure

Recently Azure has upgraded the default kernel on Ubuntu 20.04 to 5.15.0-1045-azure. This breaks the ability to use AVX on SGX enclaves.
The last known kernel that worked correctly was 5.15.0-1043-azure and therefore we'll downgrade to that kernel before we install the BlindAI-drm server.

Run the downgrade_kernel_azure.sh script to downgrade the kernel.
```bash
./downgrade_kernel_azure.sh
```

This will present a warning asking if you want to abort removing the kernel you're currently using.
Select **No** to continue removing the kernel.

![kernel warning](https://github.com/mithril-security/BlindAI_drm_fli/blob/main/docs/assets/kernel_removal_warning.png)

Once this is done, reboot the VM.
```bash
sudo reboot
```

## General pre-requisites for all parties

All parties: the custodian, AI borrower and end user can use our poetry environment to install the required BlindAI client.

To do this, you can run the following from the root of the BlindAI_drm_fli repo:

```bash
$ cd client/ && poetry shell
$ poetry install
``` 

## Demo

In this demo, we are going to show a quick example of controlled AI consumption using BlindAI DRM with the COVIDNet model, which takes images of patient chest x-rays and returns a probability of this patient having Covid.

You can download the COVIDnet model by running the following command : 
```bash
pip install gdown
gdown 1Rzl_XpV_kBw-lzu_5xYpc8briFd7fjvc
```

The model downloaded will be named as `COVID-Net-CXR-2.onnx`.

For the purposes of this demo, we do this on one machine using different terminal windows to show the view of each key party involved. In order to follow along on one machine, please open three terminals on your VM.

![terminals](https://github.com/mithril-security/BlindAI_drm_fli/blob/main/docs/assets/fli_1.png)

### Custodian launches their server

In the **custodian** window, you can launch the custodian server and upload your model with the following commands: [in red]
```bash
$ cd drm-blindai && python3 main.py --address=127.0.0.1 --upload=COVID-Net-CXR-2.onnx
```
We pass the path to the COVIDNet model with the `upload` parameter.

The custodian server will now wait for a connection attempt from the enclave server and verify it through a process called attestation before uploading the model.

Once the enclave server has been launched and verification is completed, we will see a connection is established and the model is successfully uploaded.

![custodian_launch](./assets/drm-server-launch.png)

### AI borrower launches enclave server

In the **Enclave** window : [in green] 

You can launch the enclave server and BlindAI using our `justfile` with the following command from the root of the BlindAI_drm_fli repo: 

```bash
$ BLINDAI_AZURE_DCS3_PATCH=1 just release 
```
You may need to wait a few minutes for the server to start running and connect to the DRM server.
	
On the **customer** : [in blue]

Before running the client, we will need to supply an image that will be ran by the CovidNet model. 
Let's fetch the CXR image to send to the model: 
![Image scan](https://raw.githubusercontent.com/lindawangg/COVID-Net/master/assets/ex-covid.jpeg)

```bash
wget --quiet https://raw.githubusercontent.com/lindawangg/COVID-Net/master/assets/ex-covid.jpeg
```

### End user query


```bash
$ cd drm-client && python3 main.py --address=127.0.0.1 --input=ex-covid.jpeg
```
Pass the path to the ex-covid.jpeg image to the 'input' parameter.

The end user can see the result of this request and how many more requests they can make (before they would need to request access to more queries from the custodian) in their console log.
![results](./assets/end-user-consumption.png)

> Note that if you wanted to send the end user query from a different machine, you would need to copy the enclave server's `manifest.prod.toml` file generated in the root at the repo on on build into the `client/blindai` folder with the name `manifest.toml`. This is so that the end user can verify they are sending their data to an authentic BlindAI enclave server.

### Custodian: monitoring usage and cutting access

The custodian can see the AI consumer's usage in their console log output:
![log](./assets/consumption-tracking-drm.png) 

They can cut all end user access to the model at any time by shutting down their custodian server. All new queries by end users to the model will now fail:

![cut-access](./assets/end-user-kill-switch.png)

If there is no inferences left the client will wait for the custodian to free more inferences. This is done through the endpoint `supply_inferences`. A example of a query done by the custodian can be the following: 
```
curl -k -X POST -d "number_inferences=10" https://127.0.0.1:6000/supply_inferences
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING HELP -->
## 🙋 Getting help

* Go to our [Discord](https://discord.com/invite/TxEHagpWd4) #support channel
* Report bugs by [opening an issue on our BlindAI GitHub](https://github.com/mithril-security/blindai_drm_fli/issues)
* [Book a meeting](https://calendly.com/contact-mithril-security/15mins?month=2023-03) with us


<!-- LICENSE -->
## 📜 License

Distributed under the Apache License, version 2.0. See [`LICENSE.md`](https://www.apache.org/licenses/LICENSE-2.0) for more information.

<!-- CONTACT -->
## 📇 Contact

Mithril Security - [@MithrilSecurity](https://twitter.com/MithrilSecurity) - contact@mithrilsecurity.io

Project Link: [https://github.com/mithril-security/BlindAI](https://github.com/mithril-security/blindai_drm_fli)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://github.com/alexandresanlim/Badges4-README.md-Profile#-blog- -->
[contributors-shield]: https://img.shields.io/github/contributors/mithril-security/BlindAI.svg?style=for-the-badge
[contributors-url]: https://github.com/mithril-security/BlindAI/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/mithril-security/BlindAI.svg?style=for-the-badge
[forks-url]: https://github.com/mithril-security/BlindAI/network/members
[stars-shield]: https://img.shields.io/github/stars/mithril-security/BlindAI.svg?style=for-the-badge
[stars-url]: https://github.com/mithril-security/BlindAI/stargazers
[issues-shield]: https://img.shields.io/github/issues/mithril-security/BlindAI.svg?style=for-the-badge
[issues-url]: https://github.com/mithril-security/BlindAI/issues
[license-shield]: https://img.shields.io/github/license/mithril-security/BlindAI.svg?style=for-the-badge
[license-url]: https://github.com/mithril-security/BlindAI/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/LinkedIn-0077B5?style=for-the-badge&logo=linkedin&logoColor=white&colorB=555
[linkedin-url]: https://www.linkedin.com/company/mithril-security-company/
[website-url]: https://www.mithrilsecurity.io
[website-shield]: https://img.shields.io/badge/website-000000?style=for-the-badge&colorB=555
[blog-url]: https://blog.mithrilsecurity.io/
[blog-shield]: https://img.shields.io/badge/Blog-000?style=for-the-badge&logo=ghost&logoColor=yellow&colorB=555
[product-screenshot]: images/screenshot.png
[Python]: https://img.shields.io/badge/Python-FFD43B?style=for-the-badge&logo=python&logoColor=blue
[Python-url]: https://www.python.org/
[Rust]: https://img.shields.io/badge/rust-FFD43B?style=for-the-badge&logo=rust&logoColor=black
[Rust-url]: https://www.rust-lang.org/fr
[Intel-SGX]: https://img.shields.io/badge/SGX-FFD43B?style=for-the-badge&logo=intel&logoColor=black
[Intel-sgx-url]: https://www.intel.fr/content/www/fr/fr/architecture-and-technology/software-guard-extensions.html
[Tract]: https://img.shields.io/badge/Tract-FFD43B?style=for-the-badge
[tract-url]: https://github.com/mithril-security/tract/tree/6e4620659837eebeaba40ab3eeda67d33a99c7cf

<!-- Done using https://github.com/othneildrew/Best-README-Template -->
