pub const TOTAL: u32 = 5000;
pub const SUBSET: u32 = 50;
pub const DATASET: usize = 10000;

pub fn test_set() -> Vec<[u8; 32]> {
    let mut testbed: Vec<[u8; 32]> = Vec::new();
    for i in 0..TOTAL {
        testbed.push(blake3::hash(&i.to_le_bytes()).into());
    }
    testbed.sort();
    testbed
}

pub fn gen_subset_i() -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let subset_i = rand::seq::index::sample(&mut rng, TOTAL as usize, SUBSET as usize).iter().map(|a| a as u32).collect::<Vec<u32>>();
    subset_i
}

pub fn pick_subset(subset_i: &Vec<u32>, data: &Vec<[u8; 32]>) -> Vec<[u8; 32]> {
    let mut subset = Vec::new();
    for i in subset_i.iter() {
        subset.push(data[*i as usize]);
    }
    subset
}

pub fn print_stats(name: &str, mut data: Vec<usize>) {
    let mean_sum: usize = Iterator::sum(data.iter());
    let mean = mean_sum/DATASET;
    let disp_sum: f32 = Iterator::sum(data.iter().map(|a| (mean as i32 - *a as i32).pow(2) as f32));
    let disp = disp_sum/(DATASET as f32);
    data.sort();
    let max = data.last();
    println!("{} proofs average size: {} bytes; sigma {} bytes; maximum {:?} bytes", name, mean, disp.sqrt(), max);

}

