use tfhe::{ConfigBuilder, generate_keys};
use tfhe::shortint::prelude::*;
use tfhe::shortint::parameters::PARAM_MESSAGE_2_CARRY_2_PBS_KS;

use bincode2 as bincode;

fn generate_fhe_keys() {
    let config = ConfigBuilder::all_disabled()
        .enable_custom_integers(PARAM_MESSAGE_2_CARRY_2_PBS_KS, None)
        .build();

    let (client_key, server_key) = generate_keys(config);

    let serialized_secret_key = bincode::serialize(&client_key).unwrap();
    let serialized_server_key = bincode::serialize(&server_key).unwrap();

    if let Err(e) = std::fs::write("client_key.txt", &serialized_secret_key) {
        println!(
            "Failed to write cks to path: {:?}. Error: {:?}",
            "client_key.txt", e
        );
        return;
    };

    if let Err(e) = std::fs::write("server_key.txt", serialized_server_key) {
        println!(
            "Failed to write sks to path: {:?}. Error: {:?}",
            "server_key.txt", e
        );
        return;
    };
}

fn main() {
    generate_fhe_keys()
}