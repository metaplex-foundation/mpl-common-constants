use std::env;

fn main() {
    let pubkeys: Vec<String> = env::args().collect();

    for key in pubkeys.iter().skip(1) {
        println!(
            "Raw pubkey for {}: {:?}",
            key,
            bs58::decode(key).into_vec().unwrap()
        );
    }
}
