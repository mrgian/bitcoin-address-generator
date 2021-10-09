use secp256k1::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ripemd160::Ripemd160;
use base58::*;

use rand::Rng;

fn main() {
    let private_key = random_key();
    println!("Private Key: {:x?}", private_key);

    let public_key = private_to_public_key(&private_key);
    println!("Public Key: {:x?}", public_key);

    let sha256_hashed = hash_sha256(&public_key);
    let ripemd160_hashed = hash_ripemd160(&sha256_hashed);
    let with_net_prefix = add_net_prefix(&ripemd160_hashed);

    let first_sha256 = hash_sha256(&with_net_prefix);
    let second_sha256 = hash_sha256(&first_sha256);

    let with_checksum = add_checksum(&with_net_prefix, &second_sha256);
    
    let address = with_checksum.to_base58();
    
    println!("Public Address: {}", address);  
}

fn random_key() -> [u8; 32] {
    return rand::thread_rng().gen::<[u8; 32]>();
}

fn private_to_public_key(private_key: &[u8; 32]) -> [u8; 33] {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(private_key).unwrap();
    return PublicKey::from_secret_key(&secp, &secret_key).serialize();
}

fn hash_sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.input(data);
    let mut hashed: [u8; 32] = [0; 32];
    hasher.result(&mut hashed);

    return hashed;
}

fn hash_ripemd160(data: &[u8]) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.input(data);
    let mut hashed: [u8; 20] = [0; 20];
    hasher.result(&mut hashed);

    return hashed;
}

fn add_net_prefix(data: &[u8; 20]) -> [u8; 21] {
    let mut with_net_prefix: [u8; 21] = [0; 21];

    for n in 1..21 {
        with_net_prefix[n] = data[n-1];
    } 

    return with_net_prefix;
}

fn add_checksum(data: &[u8; 21], full_checksum: &[u8; 32]) -> [u8; 25] {
    let mut with_checksum: [u8; 25] = [0; 25];

    for n in 0..21 {
        with_checksum[n] = data[n];
    }

    for n in 0..4 {
        with_checksum[n + 21] = full_checksum[n];
    }

    return with_checksum;
}
