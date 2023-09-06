use merkle_cbt::{CBMT, merkle_tree::Merge};

mod common;
mod perfect_merkle;
mod mmr;

use common::*;

fn main() {
    let mut merkles = Vec::new();
    for _ in 0..DATASET {
        merkles.push(perfect_merkle::do_merkle());
    }
    
    print_stats("Perfect static merkle", merkles);

    let mut mmr_set = Vec::new();
    for _ in 0..DATASET {
        mmr_set.push(mmr::do_merkle());
    }

    print_stats("Mountain range merkle", mmr_set);


}

