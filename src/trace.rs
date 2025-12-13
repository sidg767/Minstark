use crate::{field::F,hash_chain::HashChain};
pub struct Trace{
pub values:Vec<F>,
pub length:usize,
}
impl Trace{
pub fn from_hash_chain(chain: &HashChain) ->Self{
Self{
values:chain.trace().to_vec(),
length: chain.trace().len(),
}
}
pub fn get_row(&self, step: usize) -> F {
self.values[step]
}
}
mod tests{
use super::*;
use crate::field::BaseElement as F;
use crate::hash_chain::HashChain;
#[test]
fn test_trace_from_chain() {
let seed = F::new(1);
let mut chain = HashChain::new(seed);
chain.append(F::new(2));
chain.append(F::new(3));
let trace = Trace::from_hash_chain(&chain);
assert_eq!(trace.length, 3);
assert_eq!(trace.get_row(0), seed);
}
}
