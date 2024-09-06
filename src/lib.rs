#[cfg(feature = "devnet")]
pub mod constants {
    // staking dependencies
    pub const REWARD_CONTRACT_ID: &str = "BF5PatmRTQDgEKoXR7iHRbkibEEi83nVM38cUKWzQcTR";
    pub const DAO_PUBKEY: &str = "89wVNeyqqDaWKWtS4rbunYdsxxbe5V3VRx6g8GWNMTMt";
    pub const GOVERNANCE_PROGRAM_ID: &str = "GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw";
    pub const REALM_NAME: &str = "VSR Rewards 21";
}

#[cfg(feature = "mainnet")]
pub mod constants {
    // staking dependencies
    pub const REWARD_CONTRACT_ID: &str = "";
    pub const DAO_PUBKEY: &str = "";
    pub const GOVERNANCE_PROGRAM_ID: &str = "";
    pub const REALM_NAME: &str = "";
}
