use crate::field::F;
use crate::prover::{Proof, TraceOpening};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarkConfig {
    pub trace_length: usize,
    pub security_bits: usize,
}

impl Default for StarkConfig {
    fn default() -> Self {
        Self {
            trace_length: 0,
            security_bits: 128,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StarkProof {
    pub commitment: F,
    pub openings: Vec<TraceOpening>,
    pub length: usize,
    pub public_inputs: Vec<F>,
}

impl StarkProof {
    pub fn from_proof(proof: &Proof, public_inputs: Vec<F>) -> Self {
        Self {
            commitment: proof.root,
            openings: proof.openings.clone(),
            length: proof.length,
            public_inputs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::BaseElement as F;
    use crate::hash_chain::HashChain;
    use crate::prover::Prover;
    use crate::trace::Trace;

    #[test]
    fn test_stark_proof_wrapper_round_trip() {
        let seed = F::new(1);
        let inputs = vec![F::new(2), F::new(3)];
        let mut chain = HashChain::new(seed);
        for &input in &inputs {
            chain.append(input);
        }

        let trace = Trace::from_hash_chain(&chain);
        let prover = Prover::new(trace.length);
        let proof = prover.prove_stark(&trace, &inputs);
        let verifier = crate::verifier::Verifier::new();

        assert!(verifier.verify_stark(&proof, proof.commitment, trace.length, &inputs));
    }
}
