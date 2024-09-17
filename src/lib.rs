#[cfg(feature = "devnet")]
pub mod constants {
    // staking dependencies
    // BF5PatmRTQDgEKoXR7iHRbkibEEi83nVM38cUKWzQcTR
    pub const REWARD_CONTRACT_ID: [u8; 32] = [
        152, 47, 246, 180, 6, 167, 190, 112, 127, 106, 56, 227, 102, 180, 130, 9, 103, 162, 6, 220,
        192, 223, 113, 215, 186, 137, 46, 236, 168, 244, 52, 48,
    ];
    // HTsaJ7PYb6GKpeiMs2HsAcQTxoFrCF5zPXoTcYuxzcei
    pub const DAO_PUBKEY: [u8; 32] = [
        244, 157, 70, 175, 208, 191, 79, 124, 108, 236, 5, 135, 212, 234, 198, 220, 83, 173, 129,
        208, 134, 13, 87, 136, 190, 81, 159, 175, 51, 111, 137, 143,
    ];
    // DAO's community mint
    // 4dLsjeXUdvKNubE8RDR5krYsvrgXfQCCrm1k1VZfgULg
    pub const DAO_GOVERNING_MINT: [u8; 32] = [
        53, 226, 113, 62, 15, 5, 242, 245, 127, 108, 253, 92, 194, 244, 87, 12, 139, 229, 136, 132,
        29, 9, 7, 217, 199, 212, 126, 195, 125, 8, 73, 185,
    ];
    // GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw
    pub const GOVERNANCE_PROGRAM_ID: [u8; 32] = [
        234, 228, 53, 189, 238, 117, 183, 52, 205, 89, 62, 207, 154, 48, 75, 128, 36, 186, 40, 152,
        103, 183, 105, 177, 249, 60, 167, 187, 184, 142, 70, 254,
    ];
    pub const REALM_NAME: &str = "VSR Rewards 24";
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
