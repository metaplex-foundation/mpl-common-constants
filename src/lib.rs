#[cfg(feature = "devnet")]
pub mod constants {
    use solana_program::pubkey::Pubkey;

    // staking dependencies
    pub const REWARD_CONTRACT_ID: Pubkey =
        solana_program::pubkey!("BF5PatmRTQDgEKoXR7iHRbkibEEi83nVM38cUKWzQcTR");
    pub const DAO_PUBKEY: Pubkey =
        solana_program::pubkey!("89wVNeyqqDaWKWtS4rbunYdsxxbe5V3VRx6g8GWNMTMt");
    pub const GOVERNANCE_PROGRAM_ID: Pubkey =
        solana_program::pubkey!("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw");
    pub const REALM_NAME: &str = "VSR Rewards 21";
}

#[cfg(feature = "mainnet")]
pub mod constants {
    // staking dependencies
    pub const REWARD_CONTRACT_ID: Pubkey = solana_program::pubkey!("");
    pub const DAO_PUBKEY: Pubkey = solana_program::pubkey!("");
    pub const GOVERNANCE_PROGRAM_ID: Pubkey = solana_program::pubkey!("");
    pub const REALM_NAME: &str = "";
}
