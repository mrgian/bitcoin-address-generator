/*
* Bitcoin Address Generator
* 
* Author: Gianmatteo Palmieri (www.gian.im)
*
* Made for learning purposes only
*/

use secp256k1::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ripemd160::Ripemd160;
use base58::*;
use rand::Rng;
use itertools::Itertools;

fn main() {

    //Start by generating a 256bit random private key
    let private_key = random_key();
    println!("Private Key: {:02x}", private_key.iter().format(""));

    //Apply the Elliptic Curve Digital Signature Algorithm (ECDSA) to get the public key
    let public_key = private_to_public_key(&private_key);
    println!("Public Key: {:02x}", public_key.iter().format(""));

    //First hash the public key with the SHA256 algorithm
    let sha256_hashed = hash_sha256(&public_key);

    //Then hash it again with the RIPEMD160 algorithm
    let ripemd160_hashed = hash_ripemd160(&sha256_hashed);

    //Add the bitcoin main network byte (0x00) to the hashed key
    let with_net_prefix = add_net_prefix(&ripemd160_hashed);

    //Hash it again with SHA256 twice to get the checksum
    let first_sha256 = hash_sha256(&with_net_prefix);
    let second_sha256 = hash_sha256(&first_sha256);

    //Add the first four bytes of the checksum to the hashed key
    let with_checksum = add_checksum(&with_net_prefix, &second_sha256);
    
    //Encode the hashed key in ASCII using Base58 encoding
    let address = with_checksum.to_base58();
    
    //You've got your bitcoin wallet address!
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
