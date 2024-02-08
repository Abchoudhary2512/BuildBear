# NFT Swap Contract

## Overview

This Rust code provides a basic implementation of a trustless NFT swap contract. The contract allows two individuals to trade their NFTs in a secure manner. Users can create a swap by specifying the pair of NFT addresses and IDs. Once both parties deposit their NFTs, the swap can be completed, providing a decentralized and trustless mechanism for NFT trading.

## Usage

### Getting Started

Ensure you have Rust installed on your machine. If not, you can install it using [rustup](https://rustup.rs/).

### Running the Code

1. Clone the repository:

    ```bash
    git clone https://github.com/Abchoudhary2512/BuildBear.git
    ```

2. Navigate to the project directory:

    ```bash
    cd BuildBear
    ```

3. Run the code:

    ```bash
    cargo run
    ```

### How to Use the NFT Swap Contract

1. **Create an instance of the `NFTSwap` contract:**

    ```rust
    let mut nft_swap = NFTSwap::new();
    ```

2. **Create a swap by specifying the NFT pairs:**

    ```rust
    let nft1 = NFT {
        address: "0xContract1",
        id: 1,
    };

    let nft2 = NFT {
        address: "0xContract2",
        id: 2,
    };

    nft_swap.create_swap(nft1, nft2);
    ```

3. **Users can deposit their NFTs into the swap:**

    ```rust
    let user1_nft = NFT {
        address: "0xContract1",
        id: 1,
    };

    let user2_nft = NFT {
        address: "0xContract2",
        id: 2,
    };

    nft_swap.deposit(user1_nft);
    nft_swap.deposit(user2_nft);
    ```

4. **The swap is completed once both parties have deposited their NFTs.**
