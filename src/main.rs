use curve25519_dalek_ng::{
    ristretto::{self, RistrettoPoint},
    scalar::Scalar,
};
use elgamal::public::PublicKey;
use elgamal_ristretto as elgamal;
use rand_core::{OsRng, RngCore};
use safe_shuffle::SafeShuffler;
use sha2::Sha512;

fn main() {
    println!("Implementation notebook");

    let messages: Vec<RistrettoPoint> = (0..100)
        .map(|i| {
            ristretto::RistrettoPoint::hash_from_bytes::<Sha512>(format!("message_{i}").as_bytes())
        })
        .collect();

    let permutation = SafeShuffler::new(OsRng).shuffle((0..100).collect());

    permutation
        .iter()
        .enumerate()
        .for_each(|(i, v)| println!("{i} -> {v}"));

    let private_keys: Vec<elgamal::private::SecretKey> = (0..100)
        .map(|_| elgamal_ristretto::private::SecretKey::new(&mut OsRng))
        .collect();

    let public_keys: Vec<elgamal::public::PublicKey> =
        private_keys.iter().map(|sk| PublicKey::from(sk)).collect();

    let ciphers: Vec<elgamal::ciphertext::Ciphertext> = public_keys
        .iter()
        .zip(messages.iter())
        .map(|(pk, msg)| pk.encrypt(msg))
        .collect();

    // We sample p_1, ..., p_n randomizers
    let randomizers: Vec<Scalar> = (0..100).map(|_| Scalar::random(&mut OsRng)).collect();
}
