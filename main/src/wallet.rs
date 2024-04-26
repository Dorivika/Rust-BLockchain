use std::ops::Deref;
use std::rc::Rc;

use ring::rand;
use ring::signature::{self, EcdsaKeyPair, KeyPair};
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;

const CHECKSUM_LEN : i32 = 4;
const  VERSION: u8 = 0x00;


pub struct Wallet {
    key_pair: EcdsaKeyPair,
}

impl Wallet {
    fn newkeypair() -> Self {
        let rng = rand::SystemRandom::new();
        let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&signature::ECDSA_P256_SHA256_FIXED_SIGNING, &rng)
            .expect("Failed to generate PKCS#8 document");

        let key_pair = EcdsaKeyPair::from_pkcs8(&signature::ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref(), &rng)
            .expect("Failed to generate key pair");

        Self { key_pair }
    }

    pub fn public_key(&self) -> &[u8] {
        self.key_pair.public_key().as_ref()
    }

    pub fn make_wallet() -> Wallet {
        Wallet::newkeypair()
    }
    
    pub fn address(&self) -> Rc<[u8]> {
        let pubhashkey = pub_key_hash(self.public_key());
        let mut versioned_hash : Vec<u8> = vec![];
        versioned_hash.push(VERSION);
        let mut fullhash : Vec<u8> = vec![];
        versioned_hash.extend_from_slice(pubhashkey.deref());

        fullhash.extend_from_slice(&versioned_hash);
        let checksum = checksum(Rc::from(versioned_hash));
        fullhash.extend_from_slice(&checksum);

        return Rc::from(fullhash);
    }
}

pub fn pub_key_hash (pubkey : &[u8]) -> Rc<[u8]> {
    let mut hasher = Sha256::new();
    hasher.update(pubkey);
    let pub_key_sha256 = hasher.finalize();

    let mut hasher2 = Ripemd160::new();
    hasher2.update(pub_key_sha256);
    let public_ripmd = hasher2.finalize().to_vec();
    Rc::from(public_ripmd)
}

pub fn checksum(payload : Rc<[u8]>) -> Rc<[u8]> {
    let mut hasher = Sha256::new();
    let mut hasher2 = Sha256::new();

    hasher.update(payload);
    let first_hash = hasher.finalize().to_vec();
    hasher2.update(first_hash);
    let second_hash = &hasher2.finalize().to_vec()[0..4];
    Rc::from(second_hash)
}

fn base58encode(input : Rc<[u8]>) -> Rc<[u8]> {
    let encoded = bs58::encode(input).into_vec();
    Rc::from(encoded)
}
fn base58decode(input : Rc<[u8]>) -> Rc<[u8]> {
    let encoded = match bs58::decode(input).into_vec() {
        Ok(a) => a,
        _ => {
            println!("unable to do base58 decoding");
            std::process::exit(0);
        }
    };
    Rc::from(encoded)
}