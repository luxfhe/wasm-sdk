use sha2::{Digest, Sha256};
use tfhe::shortint::prelude::*;

const BENCH_NAME: &str = "bench_cpu_sha256";

#[no_mangle]
pub extern "C" fn bench_hash() -> u32 {
    let mut hashed: Vec<u8> = BENCH_NAME.into();
    for _i in 1..50_000 {
        hashed = Sha256::digest(&hashed).to_vec()
    }

    hashed[0] as u32
}

#[no_mangle]
pub extern "C" fn fhe_setup() -> u32 {

    tfhe::core_crypto::seeders::custom_seeder::set_custom_seeder("test2".to_string());

    let (client_key, server_key) = gen_keys(PARAM_MESSAGE_2_CARRY_2_KS_PBS);

    let left = 100_000_000_00;
    let right = 200_200_200_200;

    let ct_left = client_key.encrypt(left);
    let ct_right = client_key.encrypt(right);


    let num_loops: usize = 1000000;

    for _ in 0..num_loops {
        let _ = server_key.unchecked_add(&ct_left, &ct_right);
    }

    return num_loops as u32;
    // let cks_raw = include_bytes!("../client_key.txt");
    //
    // let config = ConfigBuilder::all_disabled()
    //     .enable_custom_integers(PARAM_MESSAGE_2_CARRY_2_PBS_KS, None)
    //     .build();
    //
    // let (client_key, server_key) = generate_keys(config);

    // let client_key: ClientKey = bincode::deserialize(cks_raw).unwrap();

    // set_server_key(server_key);
    //
    // return 0u32

    // let as_binary = hex::decode(cks_hex).unwrap();
    // let mut serialized_data = Cursor::new(as_binary);
    // let client_key: ClientKey = bincode::deserialize_from(&mut serialized_data).unwrap();
    //
    //
    // let clear_a = 27u8;
    // let clear_b = 128u8;
    //
    // let a = FheUint8::encrypt(clear_a, &client_key);
    // let b = FheUint8::encrypt(clear_b, &client_key);
    //
    // let result = a + b;
    //
    // let decrypted_result: u8 = result.decrypt(&client_key);
    //
    // let clear_result = clear_a + clear_b;
    //
    // clear_result as u32
}


// #[no_mangle]
// pub extern "C" fn fhe_add() -> u32 {
//
// }

fn main() {

    let seeder = tfhe::core_crypto::seeders::custom_seeder::set_custom_seeder("test".to_string());



    let before = std::time::Instant::now();
    let _ = fhe_setup();

    println!("elapsed: {:?}", before.elapsed().as_millis());
}
