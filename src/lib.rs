#[cfg(feature = "devnet")]
pub mod constants {
    // staking dependencies
    pub const REWARD_CONTRACT_ID: &str = "BF5PatmRTQDgEKoXR7iHRbkibEEi83nVM38cUKWzQcTR";
    pub const DAO_PUBKEY: &str = "89wVNeyqqDaWKWtS4rbunYdsxxbe5V3VRx6g8GWNMTMt";
    // DAO's community mint
    pub const DAO_GOVERNING_MINT: &str = "C83Ab2ZJ4nSFyVpSJXLkoPph4XVU6y2jgxJDczT4qddZ";
    pub const GOVERNANCE_PROGRAM_ID: &str = "GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw";
    pub const REALM_NAME: &str = "VSR Rewards 21";
    pub const FEE_RECEIVER: &str = "EzsKaQq61FLZwRaiUx7t17LWVVzsE8wRkhBghFrZGGwG";
}

#[cfg(feature = "mainnet")]
pub mod constants {
    // staking dependencies
    pub const REWARD_CONTRACT_ID: &str = "";
    pub const DAO_PUBKEY: &str = "";
    // DAO's community mint
    pub const DAO_GOVERNING_MINT: &str = "";
    pub const GOVERNANCE_PROGRAM_ID: &str = "";
    pub const REALM_NAME: &str = "";
    pub const FEE_RECEIVER: &str = "";
}
