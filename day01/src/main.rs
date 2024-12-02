use std::collections::{BinaryHeap, HashMap};
use std::fs;

fn main() {
    let mut l1: BinaryHeap<i32> = BinaryHeap::new();
    let mut l2: BinaryHeap<i32> = BinaryHeap::new();
    let mut occ: HashMap<i32, usize> = HashMap::new();

    fs::read_to_string("./input/01")
        .unwrap()
        .trim()
        .lines()
        .for_each(|s| {
            let mut iter = s.split_whitespace();
            let value1: i32 = iter.next().unwrap().parse().unwrap();
            let value2: i32 = iter.next().unwrap().parse().unwrap();
            occ.entry(value2).and_modify(|e| *e += 1).or_insert(1);
            l1.push(value1);
            l2.push(value2);
        });

    let mut dist: usize = 0;
    let mut similarity: usize = 0;

    while !l1.is_empty() {
        let v1 = l1.pop().unwrap();
        let v2 = l2.pop().unwrap();
        dist += (v1 - v2).unsigned_abs() as usize;
        similarity += (v1 as usize) * occ.get(&v1).unwrap_or(&0);
    }
    println!("Distance: {}", dist);
    println!("Similarity: {}", similarity);
}
