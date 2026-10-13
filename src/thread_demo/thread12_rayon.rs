use std::time::Instant;

use rayon::prelude::*;

pub fn main() {
    println!("--------------Rayon------------");
    test03();
}

fn test01() {
    println!("--------------test01------------");

    let nums: Vec<u64> = (0..10000000).collect();
    // 计算时间
    let start = Instant::now();
    let sum = nums.par_iter().sum::<u64>();
    println!("time: {:?}", start.elapsed());
    println!("sum1: {}", sum);

    let start = Instant::now();
    let sum = nums.iter().sum::<u64>();
    println!("time: {:?}", start.elapsed());
    println!("sum2: {}", sum);
}

fn test02() {
    println!("--------------test02------------");

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let matrix: Vec<Vec<u64>> = (1..13)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| c.to_vec())
        .collect();
    println!("{:?}", matrix);
    pool.scope(|s| {
        for (i, row) in matrix.iter().enumerate() {
            s.spawn(move |_| {
                let sum = row.iter().sum::<u64>();
                println!("Row {} sum is: {}", i, sum);
            });
        }
    });
}

fn test03() {
    println!("--------------test03------------");

    let matrix: Vec<Vec<u64>> = (1..13)
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| {
            // println!("c:{:?}", c.to_vec()); // c.to_vec() 会将 &[u64] 转换为 Vec<u64>
            c.to_vec()
        })
        .collect();

    println!("{:?}", matrix);
}
