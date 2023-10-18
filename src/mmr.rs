use ckb_merkle_mountain_range::{util::MemStore, Error, MMRStoreReadOps, Merge, Result, MMR};

use crate::common::*;

struct MergeHashes;

impl Merge for MergeHashes {
    type Item = [u8; 32];
    fn merge(left: &Self::Item, right: &Self::Item) -> Result<Self::Item> {
        Ok(blake3::hash(&[*left, *right].concat()).into())
    }
}

fn prepare_mmr(data: &Vec<[u8; 32]>, indices: Vec<usize>) -> (usize, MemStore<[u8; 32]>) {
    let store: MemStore<[u8; 32]> = MemStore::default();
    let mut mmr = MMR::<_, MergeHashes, _>::new(0, &store);
    for i in indices.iter() {
        mmr.push(data[*i]);
    }
    let mmr_size = mmr.mmr_size();
    (mmr_size as usize, store)
}

pub fn do_merkle() -> usize {
    let testbed = test_set();
    let store: MemStore<[u8; 32]> = MemStore::default();
    let mut mmr = MMR::<_, MergeHashes, _>::new(0, &store);
    let mut positions = Vec::new();
    for i in testbed.iter() {
        positions.push(mmr.push(*i).unwrap());
    }
    let mmr_size = mmr.mmr_size();

    let subset_i = gen_subset_i();
    let mut subset = pick_subset(&subset_i, &testbed);

    let root = mmr.get_root().unwrap();

    let proof = mmr.gen_proof(subset_i.iter().map(|a| positions[*a as usize]).collect::<Vec<u64>>()).unwrap();
    proof.verify(root, subset_i.iter().map(|a| positions[*a as usize]).zip(subset.iter().map(|a| *a)).collect()).unwrap();


    32 + proof.proof_items().len()*32 + subset_i.len() * 4
}
