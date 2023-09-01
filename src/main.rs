use rand::prelude::*;
use merkle_cbt::{CBMT, merkle_tree::Merge};

const TOTAL: u32 = 5000;
const SUBSET: u32 = 50;

struct MergeHashes {}

impl Merge for MergeHashes {
    type Item = [u8; 32];
    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        blake3::hash(&[*left, *right].concat()).into()
    }
}

type CBMTH = CBMT<[u8; 32], MergeHashes>;

fn main() {
    let mut testbed: Vec<[u8; 32]> = Vec::new();
    for i in 0..TOTAL {
        testbed.push(blake3::hash(&i.to_le_bytes()).into());
    }

    testbed.sort();

    let root = CBMTH::build_merkle_root(&testbed);
    
    println!("Merkle root is {:?}", root);
   
    let mut rng = rand::thread_rng();
    let subset_i = rand::seq::index::sample(&mut rng, TOTAL as usize, SUBSET as usize).iter().map(|a| a as u32).collect::<Vec<u32>>();
    println!("Included nodes: {:?}", subset_i);
    let mut subset = Vec::new();
    for i in subset_i.iter() {
        subset.push(testbed[*i as usize]);
    }
    //println!("{:?}", subset);

    let proof = CBMTH::build_merkle_proof(&testbed, &subset_i).unwrap();

    println!("correct inclusion proof: {:?}", proof.verify(&root, &subset));

    let indices = proof.indices();
    let lemmas = proof.lemmas();

    println!("Proof indices number: {:?}", indices.len());

    println!("Proof lemmas number: {:?}", lemmas.len());

    println!("Proof indices size: {:?}", indices.len()*4);

    println!("Proof lemmas size: {:?}", lemmas.len()*32);

    println!("Total overhead size: {:?}", indices.len()*4 + lemmas.len()*32);

    subset[0] = [0; 32];

    println!("incorrect inclusion proof: {:?}", proof.verify(&root, &subset));


}

