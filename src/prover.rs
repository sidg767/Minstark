use crate::{air::HashChainAir, trace::Trace, field::F};
use crate::merkle::{MerkleTree, MerkleProof};
pub struct Prover {
    air: HashChainAir,
}

impl Prover {
    pub fn new(trace_len: usize) -> Self {
        Self {
            air: HashChainAir::new(trace_len),
        }
    }

    pub fn prove(&self, trace: &Trace, inputs: &[F]) -> Proof {
        assert!(
            self.air.verify(trace, inputs),
            "Trace does not satisfy AIR constraints"
        );

        let tree = MerkleTree::new(trace.values.clone());
        let root = tree.root();
        let mut paths = Vec::new();
        for i in 0..trace.values.len() {
            paths.push(tree.prove(i));
        }

        Proof {
            root,
            merkle_paths: paths,
            length: trace.length,
        }
    }
}
pub struct Proof{
pub root:F,
pub merkle_paths: Vec<MerkleProof>,
pub length:usize,
}
mod tests{
use super::*;
use crate::field::BaseElement as F;
use crate::hash_chain::HashChain;
use crate::trace::Trace;
#[test]
fn test_prover(){
let seed=F::new(1);
let mut chain = HashChain::new(seed);
let inputs = vec![F::new(2), F::new(3)];
for &inp in &inputs{
chain.append(inp);
}
let trace = Trace::from_hash_chain(&chain);
let prover = Prover::new(trace.length);
let proof = prover.prove(&trace, &inputs);
assert_eq!(proof.length, trace.length);
println!("Proof root: {}", proof.root);
println!("Merkle path count = {}", proof.merkle_paths.len());
assert_eq!(proof.merkle_paths.len(), trace.length);
}
}
