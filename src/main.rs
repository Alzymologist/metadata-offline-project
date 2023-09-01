
fn main() {
    let mut testbed: Vec<[u8; 32]> = Vec::new();
    for i in 0..5000u32 {
        testbed.push(blake3::hash(&i.to_le_bytes()).into());
    }

    testbed.sort();

    let mut hasher = blake3::Hasher::new();
    let testbed_copy = testbed.clone();
    for i in testbed_copy.iter() {
        hasher.update(i);
    }

    let root_one = hasher.finalize();//blake3::hash(&testbed.concat());

    println!("{:?}", root_one);

    let mut chunks: Vec<blake3::Hash> = Vec::new();
    for i in 0..testbed.len() {
        chunks.push(blake3::guts::ChunkState::new((i as u64)).update(&testbed[i]).finalize(false).into());
    }

    let width = chunks.len();

    let depth = if width.is_power_of_two() {
        testbed.len().ilog2()
    } else {
        testbed.len().ilog2() + 1
    };
    println!("{:?}", depth);

    let mut old_collector: Vec<blake3::Hash> = chunks; //for this to work, chunks should be 1kb in
                                                       //size; there are projects that make chunks
                                                       //larger, not the opposite. No benefit over
                                                       //special merkle tree crate
    for i in 0..depth {
        let mut new_collector: Vec<blake3::Hash> = Vec::new();
        for j in 0..old_collector.len()/2 {
            new_collector.push(blake3::guts::parent_cv(&old_collector[j*2], &old_collector[j*2+1], i==depth-1));
        }
        if old_collector.len().trailing_zeros() == 0 {
            if let Some(a) = old_collector.last() {    
                new_collector.push(*a);
            }
        }
        old_collector = new_collector;

    }

    println!("{:?}", old_collector);
}

