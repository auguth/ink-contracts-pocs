# ink-contracts

![Validator Rewaed Contract](https://github.com/auguth/ink-contracts-pocs/actions/workflows/ci.yml/badge.svg)


## Ink! Environment Setup

Before working with the Validator Reward Contract, ensure your development environment meets the following requirements:

- Rust & C++ Compiler:

    In addition to Rust, installation requires a C++ compiler that supports C++17. Modern releases of gcc and clang, as well as Visual Studio 2019+, should work.

**Installation Steps:**

1. Install Rust Source:

    ``` rust
    rustup component add rust-src
    ```

2. Install `cargo-contract`:

    ``` rust
    cargo install --force --locked cargo-contract
    ```

## How to Use the Contract

1. Build and Deploy:

    - Clone the `ink-contract-pocs` repository containing the validator reward contract code.
    
        ```rust
        git clone https://github.com/auguth/ink-contracts-pocs.git
        ```

    - Run the setup script if applicable:

        ``` rust
        chmod +x setup.sh && ./setup.sh
        ```

    - Open `validator_reward_contract` folder

        ```rust
        cd validator_reward_contract
        ```

    - Build the contract in release mode using cargo-contract:

        ``` rust
        cargo contract build --release
        ```

    - Deploy the contract to your local or test network using the Polkadot-JS-App or Contracts UI:
        - Upload the WASM binary.
        - Initialize the contract by providing the required constructor parameter (owner).

2. To Run Test:
    ```rust
    cargo test
    ```

