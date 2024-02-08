use std::collections::HashMap;
use std::fmt;

// Define the NFT struct
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NFT {
    pub address: &'static str,
    pub id: u64,
}

// Implement Display for better readability
impl fmt::Display for NFT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NFT {{ address: {}, id: {} }}", self.address, self.id)
    }
}

// Define the NFTSwap contract
pub struct NFTSwap {
    swaps: HashMap<NFT, NFT>,
}

impl NFTSwap {
    // Constructor to create a new NFTSwap contract
    pub fn new() -> Self {
        Self {
            swaps: HashMap::new(),
        }
    }

    // Function to create a new swap
    pub fn create_swap(&mut self, nft1: NFT, nft2: NFT) {
        self.swaps.insert(nft1, nft2);
    }

    // Function to deposit an NFT into the swap
    pub fn deposit(&mut self, nft: NFT) {
        if let Some(counterpart) = self.swaps.get(&nft) {
            // Check if the counterpart exists in the swap
            // You may want to add additional checks here, e.g., ownership verification

            // Perform the swap
            println!("Swap completed! {} deposited and {} deposited.", nft, counterpart);
            self.swaps.remove(&nft);
        } else {
            println!("No matching swap found for {}.", nft);
        }
    }
}

fn main() {
    
    let mut nft_swap = NFTSwap::new();

    let nft1 = NFT {
        address: "0xContract1",
        id: 1,
    };

    let nft2 = NFT {
        address: "0xContract2",
        id: 2,
    };

    nft_swap.create_swap(nft1, nft2);

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
}
