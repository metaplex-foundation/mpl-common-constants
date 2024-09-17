#[cfg(feature = "devnet")]
pub mod constants {
    // staking dependencies
    // BF5PatmRTQDgEKoXR7iHRbkibEEi83nVM38cUKWzQcTR
    pub const REWARD_CONTRACT_ID: [u8; 32] = [
        152, 47, 246, 180, 6, 167, 190, 112, 127, 106, 56, 227, 102, 180, 130, 9, 103, 162, 6, 220,
        192, 223, 113, 215, 186, 137, 46, 236, 168, 244, 52, 48,
    ];
    // 89wVNeyqqDaWKWtS4rbunYdsxxbe5V3VRx6g8GWNMTMt
    pub const DAO_PUBKEY: [u8; 32] = [
        106, 76, 16, 235, 23, 188, 180, 88, 124, 176, 220, 27, 115, 46, 119, 76, 87, 28, 12, 199,
        217, 118, 217, 92, 5, 24, 119, 147, 26, 160, 247, 51,
    ];
    // DAO's community mint
    // C83Ab2ZJ4nSFyVpSJXLkoPph4XVU6y2jgxJDczT4qddZ
    pub const DAO_GOVERNING_MINT: [u8; 32] = [
        165, 62, 17, 245, 117, 224, 7, 129, 155, 169, 109, 81, 29, 211, 122, 26, 132, 189, 240,
        254, 37, 144, 62, 236, 113, 68, 160, 181, 129, 242, 208, 8,
    ];
    // GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw
    pub const GOVERNANCE_PROGRAM_ID: [u8; 32] = [
        234, 228, 53, 189, 238, 117, 183, 52, 205, 89, 62, 207, 154, 48, 75, 128, 36, 186, 40, 152,
        103, 183, 105, 177, 249, 60, 167, 187, 184, 142, 70, 254,
    ];
    pub const REALM_NAME: &str = "VSR Rewards 21";
    // EzsKaQq61FLZwRaiUx7t17LWVVzsE8wRkhBghFrZGGwG
    pub const FEE_RECEIVER: [u8; 32] = [
        207, 250, 238, 248, 108, 28, 218, 197, 38, 88, 2, 107, 98, 89, 111, 143, 240, 38, 219, 134,
        12, 245, 70, 184, 130, 226, 220, 115, 90, 135, 61, 223,
    ];
}

#[cfg(feature = "mainnet")]
pub mod constants {
    // staking dependencies
    pub const REWARD_CONTRACT_ID: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    pub const DAO_PUBKEY: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    // DAO's community mint
    pub const DAO_GOVERNING_MINT: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    pub const GOVERNANCE_PROGRAM_ID: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    pub const REALM_NAME: &str = "";
    pub const FEE_RECEIVER: [u8; 32] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
}
