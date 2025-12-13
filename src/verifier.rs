use crate::{prover::Proof, field::F};
use crate::merkle::MerkleTree;
pub struct Verifier;
impl Verifier{
pub fn new()->Self{
Self
}
pub fn verify(
    &self,
    proof: &Proof,
    expected_root: F,
    expected_length: usize,
    trace_values: &[F]
) -> bool {
if proof.length != expected_length {
return false;
}
if proof.root != expected_root {
return false;
}
for i in 0..proof.length {
let leaf = trace_values[i];
let path = &proof.merkle_paths[i];
if !MerkleTree::verify(proof.root, leaf, path) {
return false;
}
}
   true
}
}
mod tests {
use super::*;
use crate::prover::{Proof, Prover};
use crate::hash_chain::HashChain;
use crate::field::BaseElement as F;
use crate::trace::Trace;
#[test]
fn test_verifier(){
let seed=F::new(1);
let mut chain = HashChain::new(seed);
let inputs = vec![F::new(2), F::new(3)];
for &inp in &inputs{
chain.append(inp);
}
let trace=Trace::from_hash_chain(&chain);
let prover = Prover::new(trace.length);
let proof = prover.prove(&trace, &inputs);
let verifier = Verifier::new();
let valid =verifier.verify(&proof, proof.root, trace.length, &trace.values);
assert!(valid);
}
}
#[test]
fn test_end_to_end_proof() {
    use crate::hash_chain::HashChain;
    use crate::field::BaseElement as F;
    use crate::trace::Trace;
    use crate::prover::Prover;
    use crate::verifier::Verifier;

    let seed = F::new(1);
    let mut chain = HashChain::new(seed);
    chain.append(F::new(2));
    chain.append(F::new(3));

    let trace = Trace::from_hash_chain(&chain);
    let prover = Prover::new(trace.length);
    let proof = prover.prove(&trace, &[]);

    let verifier = Verifier::new();
    assert!(
        verifier.verify(&proof, proof.root, trace.length, &trace.values)
    );
}
