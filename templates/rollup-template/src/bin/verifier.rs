//! This binary implements the verification logic for the rollup. This is the code that runs inside
//! of the zkvm in order to generate proofs for the rollup.
use rollup_template::da::new_da_verifier;
use rollup_template::zkvm::ZkvmGuest;
use template_stf::{zk_stf, RollupVerifier};

fn main() {
    let guest = ZkvmGuest::new();
    let mut stf_verifier = RollupVerifier::new(zk_stf(), new_da_verifier());
    stf_verifier
        .run_block(guest)
        .expect("Prover must be honest");
}