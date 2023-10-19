# Multi-Vest-DAO

Multi-Vest-DAO is a decentralized autonomous organization (DAO) designed to fund promising projects within the MultiverseX ecosystem. Its primary goal is to strengthen the community and support the development of the ecosystem by allocating resources to projects with high potential. This README provides an overview of the project, how to get started, and how to contribute.

## Table of Contents
- [Multi-Vest-DAO](#multi-vest-dao)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)

## Features

- **Project Funding:** Multi-Vest-DAO empowers the community to vote and fund projects that align with the goals of the MultiverseX ecosystem.

- **Decentralized Governance:** Decisions, including project funding, are made through a decentralized and transparent governance process.

- **Staking:** Token holders can stake their tokens to participate in governance and earn rewards.

- **Transparency:** All proposals, votes, and fund allocation details are recorded on the blockchain for public scrutiny.

## Getting Started

Follow these instructions to get a copy of the project up and running on your local machine.

### Prerequisites

- [Rust](https://www.rust-lang.org/) - Ensure you have Rust installed.
-  **Important** use `nightly` rust version
-  follow instructions [here](https://docs.multiversx.com/developers/tutorials/your-first-dapp#software-prerequisites) 

### Installation

1. Clone this repository:
   ```shell
   git clone git@github.com:EmanHerawy/multi-vest-dao.git
    ```
2. Build the project:
3. ```shell
   cargo build --release
 
   ```bah
   #deploy the pallet to devnet 
   mxpy --verbose contract deploy  --bytecode  <your contract wasm file path>  --pem <path to your pem>  --recall-nonce  --gas-limit 60000000  --arguments <set the staking minimum>   --chain D  --proxy https://devnet-api.multiversx.com  --outfile deploy-devnet.interaction.json  --send
    ```
