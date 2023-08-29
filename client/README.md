<a name="readme-top"></a>



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/mithril-security/blindai">
    <img src="https://github.com/mithril-security/blindai/raw/main/docs/assets/logo.png" alt="Logo" width="80" height="80">
  </a>

<h1 align="center">BlindAI DRM Proof-Of-Concept</h1>

[![Website][website-shield]][website-url]
[![Blog][blog-shield]][blog-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

  <p align="center">
    <b>BlindAI DRM Proof-Of-Concept</b> is an <b>AI privacy solution</b>, allowing users to query popular AI models or serve their own models whilst ensuring that users' data remains private every step of the way.
    It includes the code for a DRM server/client that verifies the number of inferences. 
	<br /><br />
    <a href="https://blindai.mithrilsecurity.io/en/latest"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://blindai.mithrilsecurity.io/en/latest/docs/getting-started/quick-tour/">Try Demo</a>
    ·
    <a href="https://github.com/mithril-security/blindai/issues">Report Bug</a>
    ·
    <a href="https://github.com/mithril-security/blindai/issues">Request Feature</a>
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

**BlindAI DRM POC** is an Open-Source solution to query and deploy AI models while **guaranteeing data privacy**. It includes a DRM Server and client to monitor inference consumption of a particular model.



You can find our more about BlindAI API and BlindAI Core [here](https://blindai.mithrilsecurity.io/en/latest/docs/getting-started/blindai_structure/).

### Built With 

[![Rust][Rust]][Rust-url] [![Python][Python]][Python-url] [![Intel-SGX][Intel-SGX]][Intel-sgx-url] [![Tract][Tract]][tract-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Set up

### Choosing the VM
There is different VMs available on Azure for running confidential computing applications. 

As we are working oon intel SGX, we are going to choose the DCs v3 family that supports Intel SGX (and more precisely SGX 2). To have enough memory to run our models, we choose the 64gb memory with 8-vcpus.
![Azure VM](https://github.com/mithril-security/blindai_drm_fli/blob/main/docs/assets/set_up.png)

After the creation of the instance, we can connect to it via SSH. The command and methods are usually explained at the  connect section tab.

### Setting up Intel SGX and the needed dependencies 
After connecting to the instance via SSH you can run the following scripts to install SGX, rust, and all the configuration needed to run our BlindAI secure enclave. 

We begin by cloning the BlindAI DRM repo via github :
```bash
$ git clone https://github.com/mithril-security/blindai_drm_fli.git
$ cd blindai_drm_fli/
```

- The first script installs all the dependencies needed for SGX and remote attestation to work perfectly, this one should be ran as root : 
```bash 
$ sudo ./install_packages.sh
``` 

- Then, run the following script in normal user to finish the intallation configuration.
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

Lastly, we need to install the blindAI client so that it can be used on the DRM. 
```bash
$ cd client/ && poetry shell
$ poetry install
``` 
And the blindAI should install on the poetry environment

_For more examples on the BlindAI project, you can refer to the [Documentation](https://blindai.mithrilsecurity.io/en/latest/)_

## Demo

In this quick demo, we are going to use the ResNet model. 
You can download the resnet model chosen directly from the onnx repo here : [https://github.com/onnx/models/tree/main/vision/classification/resnet/model](). 
The one that will be used on this demo is `resnet101-v2-7.onnx`.

Open 3 terminals.
(Either through VSCode or powershell)

![terminals](https://github.com/mithril-security/blindai_drm_fli/blob/main/docs/assets/fli_1.png)

On the **custodian** run the following commands: [in red]
```bash
$ cd client && poetry shell
$ cd ../drm-blindai && python3 main.py --address=127.0.0.1 --upload=resnet101-v2-7.onnx
```

The second line will run the DRM server and wait for the enclave to be ready.
	

on the **Enclave** : [in green] 
	run the enclave and blindai by typing : 
```bash
$ BLINDAI_AZURE_DCS3_PATCH=1 just release 
```
A few seconds are required for the app to run and should connect to the DRM server. 
	
On the **customer** : [in blue]

Before running the client, we will need to supply an image that will be ran by the ResNet model. 
We choose the following one for the example: 
![dog.jpg](https://github.com/pytorch/hub/raw/master/images/dog.jpg)

That you can download by entering the following command: 
```bash
$ wget https://github.com/pytorch/hub/raw/master/images/dog.jpg
```
We can then supply it directly to the client:

```bash
$ cd client && poetry shell
$ cd ../drm-client && python3 main.py --address=127.0.0.1 --input=dog.jpg
```




<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING HELP -->
## 🙋 Getting help

* Go to our [Discord](https://discord.com/invite/TxEHagpWd4) #support channel
* Report bugs by [opening an issue on our BlindAI GitHub](https://github.com/mithril-security/blindai/issues)
* [Book a meeting](https://calendly.com/contact-mithril-security/15mins?month=2023-03) with us


<!-- LICENSE -->
## 📜 License

Distributed under the Apache License, version 2.0. See [`LICENSE.md`](https://www.apache.org/licenses/LICENSE-2.0) for more information.


<!-- CONTACT -->
## 📇 Contact

Mithril Security - [@MithrilSecurity](https://twitter.com/MithrilSecurity) - contact@mithrilsecurity.io

Project Link: [https://github.com/mithril-security/blindai](https://github.com/mithril-security/blindai)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://github.com/alexandresanlim/Badges4-README.md-Profile#-blog- -->
[contributors-shield]: https://img.shields.io/github/contributors/mithril-security/blindai.svg?style=for-the-badge
[contributors-url]: https://github.com/mithril-security/blindai/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/mithril-security/blindai.svg?style=for-the-badge
[forks-url]: https://github.com/mithril-security/blindai/network/members
[stars-shield]: https://img.shields.io/github/stars/mithril-security/blindai.svg?style=for-the-badge
[stars-url]: https://github.com/mithril-security/blindai/stargazers
[issues-shield]: https://img.shields.io/github/issues/mithril-security/blindai.svg?style=for-the-badge
[issues-url]: https://github.com/mithril-security/blindai/issues
[license-shield]: https://img.shields.io/github/license/mithril-security/blindai.svg?style=for-the-badge
[license-url]: https://github.com/mithril-security/blindai/blob/master/LICENSE.txt
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
