use crate::field::F;
use crate::field_traits::FieldElement;
use crate::poseidon_constants::{ALPHA, MDS, NUM_FULL_ROUNDS, NUM_PARTIAL_ROUNDS, ROUND_CONSTANTS};
fn add_round_constants(state: &mut [F; 3], round: usize) {
    for i in 0..3 {
        state[i] += ROUND_CONSTANTS[round][i];
    }
}
fn apply_sbox(state: &mut [F; 3], full_round: bool) {
    if full_round {
        for i in 0..3 {
            state[i] = state[i].pow(ALPHA);
        }
    } else {
        state[0] = state[0].pow(ALPHA);
    }
}
fn mds_mix(state: &mut [F; 3]) {
    let old = *state;
    for i in 0..3 {
        let mut acc = F::ZERO;
        for j in 0..3 {
            acc += MDS[i][j] * old[j];
        }
        state[i] = acc;
    }
}
pub fn permute(state: &mut [F; 3]) {
    let total_rounds = NUM_FULL_ROUNDS + NUM_PARTIAL_ROUNDS;
    for round in 0..total_rounds {
        add_round_constants(state, round);
        let is_full =
            !(NUM_FULL_ROUNDS / 2..NUM_FULL_ROUNDS / 2 + NUM_PARTIAL_ROUNDS).contains(&round);
        apply_sbox(state, is_full);
        mds_mix(state);
    }
}
pub fn poseidon_hash2(a: F, b: F) -> F {
    let mut state = [a, b, F::ZERO];
    permute(&mut state);
    state[0]
}
