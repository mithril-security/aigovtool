# Contributing to BlindAI 
_________________________________

Welcome and thank you for taking the time to contribute to BlindAI! 🎉🎉

The following guide is a set of guidelines to help you contribute to the [BlindAI](https://github.com/mithril-security/blindai) project. These are mostly advice, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## 📝 Code of conduct
____________________________

This project and everyone participating in it is governed by the [Mithril Security Code Of Conduct](code_of_conduct.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to [contact@mithrilsecurity.io](mailto:contact@mithrilsecurity.io).

## 🚀 What should I know before I get started?
____________________________

### ❓ How to ask a question?
If you have a question to ask or if you want to open a discussion about BlindAI or privacy in data science in general, we have a dedicated [Discord Community](https://discord.gg/TxEHagpWd4) in which all these kind of exchanges are more than welcome!

### ⚙️ The BlindAI project

BlindAI is an **open-source solution** allowing users to query popular AI models or serve their own models with **assurances that users' private data will remain private**. The querying of models is done via our **easy-to-use BlindAI Python library**.

Data sent by users to the AI model is kept **confidential at all times**. Neither the AI service provider nor the Cloud provider (if applicable), can see the data.

Confidentiality is assured by hardware-enforced **Trusted Execution Environments**. We explain how they keep data and models safe in detail [here](https://blindai.mithrilsecurity.io/en/latest/docs/getting-started/confidential_computing/).

There are two main scenarios for BlindAI:

- **BlindAI**: Using BlindAI to query popular AI models hosted by Mithril Security.
- **BlindAI.Core**: Using BlindAI's underlying technology to host your own BlindAI server instance to securely deploy your own models.

You can find our more about BlindAI and BlindAI.Core [here](https://blindai.mithrilsecurity.io/en/latest/docs/getting-started/blindai_structure/).

### 📁 BlindAI project structure
```sh
BlindAI Project ⚙️🔒/
├─ client/
│  ├─ blindai/ # client src files
├─ dev-container-azure/ # config for Azure dev container
├─ docs/
├─ src/ # server src files
├─ runner/ # launches enclave and handles remote attestation
├─ tests/
├─ ring-fornatix/, rouille/, tract/, tar-rs-sgx/, tiny-http/ # patched external library submodules
```

### 📚 Useful resources
We highly encourage you to take a look at this resources for further information about BlindAI ⚙️🔒. 

* [Documentation - BlindAI official documentation](https://blindai.readthedocs.io)
* [Blog - Mithril Security blog](https://blog.mithrilsecurity.io/)

## 💻 Contributing code
____________________________

This section presents the different options that you can follow in order to contribute to the BlindAI🚀🔐 project. You can either **Report Bugs**, **Suggest Enhancements** or **Open Pull Requests**.

### 🐞 Reporting bugs
This section helps you through reporting Bugs for BlindAI. Following the guidelines helps the maintainers to understand your report, reproduce the bug and work on fixing at as soon as possible. 

!!! bug "Important!"

	Before reporting a bug, please take a look at the [existing issues](https://github.com/mithril-security/BlindAI/issues). You may find that the bug has already been reported and that you don't need to create a new one.

#### How to report a bug? 
To report a Bug, you can either:

- Follow this [link](https://github.com/mithril-security/blindai/issues/new?assignees=&labels=&template=bug-report.md&title=) and fill the bug report with the required information.

- Go to BlindAI GitHub repository:

	* Go to `Issues` tab.
	* Click on `New Issue` button.
	* Choose the `Bug` option.
	* Fill the report with the required information.

#### How to submit a good bug report?
- Follow the [bug report template](https://github.com/mithril-security/blindai/issues/new?assignees=&labels=&template=bug-report.md&title=) as much as possible (*You can add further details if needed*).
- Use a clear and descriptive title.
- Describe the expected behavior, the behavior that's actually happening, and how often it reproduces.
- Describe the exact steps to reproduce the problem.
- Specify the versions of BlindAI Client (and server if using BlindAI.Core) that produced the bug.
- Add any other relevant information about the context, your development environment (*operating system, language version, Libtorch version, platform, etc*).
- Attach screenshots, code snippets and any helpful resources.  

### 💯 Suggesting enhancements 
This section guides you through suggesting enhancements for the BlindAI project. You can suggest one or many by opening a **GitHub Issue**. 

!!! example "Important!"

	Before opening an issue, please take a look at the [existing issues](https://github.com/mithril-security/blindai/issues). You may find that the same suggestion has already been done and that you don't need to create a new one.

#### How to suggest an enhancement? 
To suggest enhancement for BlindAI Project, you can either:

- Follow this [link](https://github.com/mithril-security/blindai/issues/new/choose), choose the most relevant option and fill the report with the required information.

- Go to BlindAI GitHub repository:

  * Go to `Issues` tab.
  * Click on `New Issue` button.
  * Choose the most relevant option.
  * Fill the description with the required information.

#### How to submit a good enhancement suggestion?
- Choose the most relevant issue type for your suggestion.
- Follow the provided template as much as possible.
- Use a clear and descriptive title.
- Add any other relevant information about the context, your development environment (*operating system, language version, etc*).
- Attach screenshots, code snippets and any helpful resources. 

### 💎 Pull requests
This section helps you through the process of opening a pull request and contributing with code to BlindAI!

#### How to open a pull request? 
- Go to BlindAI GitHub repository.
- Fork BlindAI project.
- [Setup your local development environment.](#setting-your-local-development-environment)
- Do your magic ✨ and push your changes. 
- Open a pull request.
- Fill the description with the required information.

#### How to submit a good pull request?
- Make sure your pull request solves an open issue or fixes a bug. If no related issue exists, please consider opening an issue first so that we can discuss your suggestions. 
- Follow the [style guidelines](#style-guidelines). 
- Make sure to use a clear and descriptive title.
- Follow the instructions in the pull request template.
- Provide as many relevant details as possible.
- Make sure to [link the related issues](https://docs.github.com/en/issues/tracking-your-work-with-issues/about-issues#efficient-communication) in the description.

!!! warning "Important!"

	While the prerequisites above must be satisfied prior to having your pull request reviewed, the reviewer(s) may ask you to complete additional work, tests, or other changes before your pull request can be accepted.

### 🛠️ Setting your local development environment
You can find instructions of how to set up and install everything you need to run the BlindAI server on a VM or on your local SGX2-ready machine in the [official documentation](../../tutorials/core/installation.md).

Once your machine is set up, you can check our guide on how to [Setup your local development environment](#setting-your-local-development-environment) which will install create a working environment with all necessary dependencies to work on BlindAI. 

If you encounter any difficulties with that, don't hesitate to reach out to us through [Discord](https://discord.gg/TxEHagpWd4) and ask your questions. 

## 🏷️ Issue tracker tags
____________________________

Issue type tags:

|             |                                                             |
| ----------- | ----------------------------------------------------------- |
| question    | Any questions about the project                             |
| bug         | Something isn't working                                     |
| enhancement | Improving performance, usability, consistency               |
| docs        | Documentation, tutorials, and example projects              |
| new feature | Feature requests or pull request implementing a new feature |
| test        | Improving unit test coverage, e2e test, CI or build         |