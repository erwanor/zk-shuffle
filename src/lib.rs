use merlin;
use rand_core::{CryptoRng, RngCore};

// some homomorphic cipher here
struct Cipher {}

struct Proof {}

struct BayerGrothShuffler<R: CryptoRng + RngCore> {
    num_columns: usize,
    num_rows: usize,
    rows: Vec<Cipher>,
    columns: Vec<Cipher>,
    internal_shuffler: safe_shuffle::SafeShuffler<R>,
    internal_rng: R,
    transcript: merlin::Transcript,
    verifier: BayerGrothVerifier,
}

struct BayerGrothVerifier {}

enum ProverError {
    SomethingSomething,
}

impl<R> BayerGrothShuffler<R>
where
    R: CryptoRng + RngCore,
{
    fn new() -> Self {
        unimplemented!()
    }

    fn read_input(&mut self, ciphers: Vec<Cipher>) -> Result<Self, ProverError> {
        unimplemented!()
    }

    fn shuffle(&mut self, rounds: usize) -> Result<Proof, ProverError> {
        unimplemented!()
    }
}

impl BayerGrothVerifier {
    fn new() -> Self {
        unimplemented!()
    }

    fn verify_precompute(&self, proof: Proof) -> bool {
        unimplemented!()
    }

    fn verify(proof: Proof) -> bool {
        false
    }
}
