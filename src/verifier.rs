use crate::air::HashChainAir;
use crate::merkle::MerkleTree;
use crate::stark::StarkProof;
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
        inputs: &[F],
    ) -> bool {
        if proof.length != expected_length {
            return false;
        }
        if proof.root != expected_root {
            return false;
        }
        if proof.openings.len() != expected_length {
            return false;
        }
        let mut trace_values = Vec::with_capacity(proof.openings.len());
        for (index, opening) in proof.openings.iter().enumerate() {
            if opening.proof.index != index
                || !MerkleTree::verify(proof.root, opening.value, &opening.proof)
            {
                return false;
            }
            trace_values.push(opening.value);
        }
        let trace = crate::trace::Trace {
            values: trace_values.to_vec(),
            length: expected_length,
        };
        HashChainAir::new(expected_length).verify(&trace, inputs)
    }

    pub fn verify_stark(
        &self,
        proof: &StarkProof,
        expected_root: F,
        expected_length: usize,
        inputs: &[F],
    ) -> bool {
        if proof.public_inputs.len() != inputs.len() {
            return false;
        }
        if proof
            .public_inputs
            .iter()
            .zip(inputs.iter())
            .any(|(a, b)| a != b)
        {
            return false;
        }

        let legacy_proof = Proof {
            root: proof.commitment,
            openings: proof.openings.clone(),
            length: proof.length,
        };
        self.verify(&legacy_proof, expected_root, expected_length, inputs)
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
        let valid = verifier.verify(&proof, proof.root, trace.length, &inputs);
        assert!(valid);
    }

    #[test]
    fn test_verifier_rejects_tampered_opening() {
        let seed = F::new(1);
        let mut chain = HashChain::new(seed);
        let inputs = vec![F::new(2), F::new(3)];
        for &input in &inputs {
            chain.append(input);
        }
        let trace = Trace::from_hash_chain(&chain);
        let prover = Prover::new(trace.length);
        let mut proof = prover.prove(&trace, &inputs);
        proof.openings[1].value += F::new(1);

        let verifier = Verifier::new();
        assert!(!verifier.verify(&proof, proof.root, trace.length, &inputs));
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
    assert!(verifier.verify(&proof, proof.root, trace.length, &inputs));
}
