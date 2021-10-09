use secp256k1::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use crypto::ripemd160::Ripemd160;
use hex::FromHex;
use base58::*;

fn main() {
    let private256 = "03902e4f09664bc177fe4e090dcd9906b432b50f15fb6151984475c1c75c35b6";
    let private_bytes = <[u8; 32]>::from_hex(private256).unwrap();

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&private_bytes).unwrap();
    let verifying_key = PublicKey::from_secret_key(&secp, &secret_key).serialize();

    println!("Secret Key: {:x?}", verifying_key);

    let mut sha256_hasher = Sha256::new();
    sha256_hasher.input(&verifying_key);
    let mut sha256_hashed: [u8; 32] = [0; 32];
    sha256_hasher.result(&mut sha256_hashed);

    let mut ripemd160_hasher = Ripemd160::new();
    ripemd160_hasher.input(&sha256_hashed);
    let mut ripemd160_hashed: [u8; 20] = [0; 20];
    ripemd160_hasher.result(&mut ripemd160_hashed);

    println!("Hashed: {:x?}", ripemd160_hashed);

    let mut withnet: [u8; 21] = [0; 21];

    for n in 1..21 {
        withnet[n] = ripemd160_hashed[n-1];
    } 

    println!("With Net: {:x?}", withnet);


    let mut sha1_hasher = Sha256::new();
    sha1_hasher.input(&withnet);
    let mut sha1_hashed: [u8; 32] = [0; 32];
    sha1_hasher.result(&mut sha1_hashed);

    let mut sha2_hasher = Sha256::new();
    sha2_hasher.input(&sha1_hashed);
    let mut sha2_hashed: [u8; 32] = [0; 32];
    sha2_hasher.result(&mut sha2_hashed);

    println!("2SHA: {:x?}", sha2_hashed); 
    
    let mut withcheck: [u8; 25] = [0; 25];

    for n in 0..21 {
        withcheck[n] = withnet[n];
    }

    for n in 0..4 {
        withcheck[n + 21] = sha2_hashed[n];
    }

    println!("With Check: {:x?}", withcheck);

    let public_key = withcheck.to_base58();
    println!("Public Key: {}", public_key);
}
