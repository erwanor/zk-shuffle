use curve25519_dalek_ng::{
    ristretto::{self, RistrettoPoint},
    scalar::Scalar,
    traits::Identity,
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

    let private_key = elgamal_ristretto::private::SecretKey::new(&mut OsRng);
    let public_key = PublicKey::from(&private_key);

    // We sample p_1, ..., p_n randomizers
    let randomizers: Vec<Scalar> = (0..100).map(|_| Scalar::random(&mut OsRng)).collect();

    let ciphers: Vec<elgamal::ciphertext::Ciphertext> = messages
        .iter()
        .zip(randomizers.clone().into_iter())
        .map(|(msg, p)| public_key.encrypt_with_blinding_factor(msg, p))
        .collect();

    // Shuffle ciphers: C*_i = C_{pi(i)} * Encrypt(1, P_i)
    let shuffled_ciphers: Vec<elgamal::ciphertext::Ciphertext> = permutation
        .into_iter()
        .enumerate()
        .map(|(i, new_i)| {
            let p_i = randomizers[i];
            ciphers[new_i]
                + public_key.encrypt_with_blinding_factor(&RistrettoPoint::identity(), p_i)
        })
        .collect();
}
