use crate::{field::F, poseidon::poseidon_hash2};
pub struct MerkleTree {
    leaves: Vec<F>,
    nodes: Vec<Vec<F>>, // nodes[level][index]
    root: F,
}

/// A Merkle authentication path for a single leaf.
pub struct MerkleProof {
    pub path: Vec<F>,
    pub index: usize,
}

impl MerkleTree {
    /// Create a Merkle tree from a list of leaves.
    /// Leaves length must be > 0 and will be padded to next power of two.
    pub fn new(leaves: Vec<F>) -> Self {
        assert!(!leaves.is_empty(), "no leaves provided");

        // pad to power of two
        let size = leaves.len().next_power_of_two();
        let mut padded = leaves.clone();
        padded.resize(size, *padded.last().unwrap());

        // level 0 = leaves
        let mut nodes = vec![padded.clone()];

        // build internal levels
        while nodes.last().unwrap().len() > 1 {
            let prev = nodes.last().unwrap();
            let mut next = Vec::with_capacity(prev.len() / 2);

            for i in (0..prev.len()).step_by(2) {
                let left = prev[i];
                let right = prev[i + 1];
                next.push(poseidon_hash2(left, right));
            }
            nodes.push(next);
        }

        let root = nodes.last().unwrap()[0];

        Self {
            leaves,
            nodes,
            root,
        }
    }

    /// Return Merkle root.
    pub fn root(&self) -> F {
        self.root
    }

    /// Return the full authentication path (all siblings) for a leaf index.
    pub fn prove(&self, leaf_index: usize) -> MerkleProof {
        assert!(leaf_index < self.leaves.len());

        let mut index = leaf_index;
        let mut path = Vec::new();

        for level in 0..(self.nodes.len() - 1) {
            let sibling = if index.is_multiple_of(2) {
                self.nodes[level][index + 1]
            } else {
                self.nodes[level][index - 1]
            };
            path.push(sibling);
            index /= 2;
        }

        MerkleProof {
            path,
            index: leaf_index,
        }
    }

    /// Verify a Merkle proof for a given leaf.
    pub fn verify(root: F, leaf: F, proof: &MerkleProof) -> bool {
        let mut hash = leaf;
        let mut idx = proof.index;

        for sibling in &proof.path {
            if idx.is_multiple_of(2) {
                hash = poseidon_hash2(hash, *sibling);
            } else {
                hash = poseidon_hash2(*sibling, hash);
            }
            idx /= 2;
        }

        hash == root
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::BaseElement as F;

    #[test]
    fn test_merkle_basic() {
        let leaves = vec![F::new(1), F::new(2), F::new(3), F::new(4)];
        let tree = MerkleTree::new(leaves.clone());

        let root = tree.root();
        assert!(tree.nodes.len() >= 2);

        for i in 0..leaves.len() {
            let proof = tree.prove(i);
            let ok = MerkleTree::verify(root, leaves[i], &proof);
            assert!(ok, "proof failed for index {}", i);
        }
    }

    #[test]
    fn test_merkle_non_power_of_two() {
        let leaves = vec![F::new(10), F::new(20), F::new(30)];
        let tree = MerkleTree::new(leaves.clone());

        let root = tree.root();
        for i in 0..leaves.len() {
            let proof = tree.prove(i);
            assert!(MerkleTree::verify(root, leaves[i], &proof));
        }
    }
}
