use merkle_cbt::{CBMT, merkle_tree::Merge};

use crate::common::*;

struct MergeHashes {}

impl Merge for MergeHashes {
    type Item = [u8; 32];
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        blake3::hash(&[*left, *right].concat()).into()
    }
}

type CBMTH = CBMT<[u8; 32], MergeHashes>;

pub fn do_merkle() -> usize {
    let testbed = test_set();

    let root = CBMTH::build_merkle_root(&testbed);
    
    let subset_i = gen_subset_i();
    let mut subset = pick_subset(&subset_i, &testbed);

    let proof = CBMTH::build_merkle_proof(&testbed, &subset_i).unwrap();

    assert!(proof.verify(&root, &subset));

    let indices = proof.indices();
    let lemmas = proof.lemmas();

    //each index is a u32, each lemma is H256, and there is a H256 root
    let result = indices.len()*4 + lemmas.len()*32 + 32;
    subset[0] = [0; 32];
    assert!(!proof.verify(&root, &subset));

    result
}

