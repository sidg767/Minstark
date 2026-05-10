use crate::field::F;
use crate::poseidon::poseidon_hash2;

pub struct HashChain {
    trace: Vec<F>,
}

impl HashChain {
    pub fn new(seed: F) -> Self {
        Self { trace: vec![seed] }
    }

    pub fn append(&mut self, value: F) {
        let last = *self.trace.last().unwrap();
        self.trace.push(poseidon_hash2(last, value));
    }

    pub fn trace(&self) -> &[F] {
        &self.trace
    }

    pub fn tip(&self) -> F {
        *self.trace.last().unwrap()
    }
}
#[test]
fn test_hash_chain_growth() {
    use crate::field::BaseElement as F;
    use crate::hash_chain::HashChain;

    let seed = F::new(1);
    let mut chain = HashChain::new(seed);

    chain.append(F::new(2));
    chain.append(F::new(3));

    let trace = chain.trace();

    assert_eq!(trace.len(), 3);
    assert_eq!(trace[0], seed);
}
