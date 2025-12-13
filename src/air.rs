use crate::field_traits::FieldElement;
use crate::{field::F, trace::Trace};
pub struct HashChainAir{
pub trace_len: usize,
}
impl HashChainAir{
pub fn new(trace_len: usize) -> Self {
Self{trace_len}
}
pub fn evaluate_transition(&self,trace:&Trace,inputs:&[F])->Vec<F>{
let mut constraints =Vec::with_capacity(trace.length - 1);
for i in 0..(trace.length - 1){
let expected = crate::poseidon::poseidon_hash2(trace.get_row(i), inputs[i]);
constraints.push(trace.get_row(i + 1) - expected);
}
constraints
}
pub fn verify(&self, trace: &Trace, inputs: &[F])->bool{
self.evaluate_transition(trace, inputs)
.iter()
.all(|c| *c == F::ZERO)
}
}
mod tests{
use super::*;
use crate::field::BaseElement as F;
use crate::hash_chain::HashChain;
fn test_air_constraints(){
let seed = F::new(1);
let mut chain = HashChain::new(seed);
let inputs = vec![F::new(2), F::new(3)];
for &inp in &inputs{
chain.append(inp);
}
let trace = Trace::from_hash_chain(&chain);
let air = HashChainAir::new(trace.length);
assert!(air.verify(&trace, &inputs));
}
}
#[test]
fn test_air_accepts_valid_trace() {
    use crate::hash_chain::HashChain;
    use crate::field::BaseElement as F;
    use crate::trace::Trace;

    let seed = F::new(1);
    let mut chain = HashChain::new(seed);
    chain.append(F::new(2));
    chain.append(F::new(3));

    let trace = Trace::from_hash_chain(&chain);
    let air = crate::air::HashChainAir::new(trace.length);

    assert!(air.verify(&trace, &[]));
}
