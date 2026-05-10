use crate::air::HashChainAir;
use crate::merkle::MerkleTree;
use crate::{field::F, prover::Proof};
pub struct Verifier;
impl Verifier {
    pub fn new() -> Self {
        Self
    }
    pub fn verify(
        &self,
        proof: &Proof,
        expected_root: F,
        expected_length: usize,
        trace_values: &[F],
        inputs: &[F],
    ) -> bool {
        if proof.length != expected_length {
            return false;
        }
        if proof.root != expected_root {
            return false;
        }
        if trace_values.len() != expected_length {
            return false;
        }
        for i in 0..proof.length {
            let leaf = trace_values[i];
            let path = &proof.merkle_paths[i];
            if !MerkleTree::verify(proof.root, leaf, path) {
                return false;
            }
        }
        let trace = crate::trace::Trace {
            values: trace_values.to_vec(),
            length: expected_length,
        };
        HashChainAir::new(expected_length).verify(&trace, inputs)
    }
}
mod tests {
    use super::*;
    use crate::field::BaseElement as F;
    use crate::hash_chain::HashChain;
    use crate::prover::Prover;
    use crate::trace::Trace;

    #[test]
    fn test_verifier() {
        let seed = F::new(1);
        let mut chain = HashChain::new(seed);
        let inputs = vec![F::new(2), F::new(3)];
        for &inp in &inputs {
            chain.append(inp);
        }
        let trace = Trace::from_hash_chain(&chain);
        let prover = Prover::new(trace.length);
        let proof = prover.prove(&trace, &inputs);
        let verifier = Verifier::new();
        let valid = verifier.verify(&proof, proof.root, trace.length, &trace.values, &inputs);
        assert!(valid);
    }
}
#[test]
fn test_end_to_end_proof() {
    use crate::field::BaseElement as F;
    use crate::hash_chain::HashChain;
    use crate::prover::Prover;
    use crate::trace::Trace;
    use crate::verifier::Verifier;

    let seed = F::new(1);
    let mut chain = HashChain::new(seed);
    chain.append(F::new(2));
    chain.append(F::new(3));

    let inputs = vec![F::new(2), F::new(3)];
    let trace = Trace::from_hash_chain(&chain);
    let prover = Prover::new(trace.length);
    let proof = prover.prove(&trace, &inputs);

    let verifier = Verifier::new();
    assert!(verifier.verify(&proof, proof.root, trace.length, &trace.values, &inputs));
}
