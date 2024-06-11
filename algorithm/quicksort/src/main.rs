use rand::prelude::*;
use std::time::{Duration, Instant};


fn main() {
    const SIZE: usize = 10000000;
    let mut arr: Vec<u32> = (0..SIZE).map(|_| rand::thread_rng().gen_range(1..10000000)).collect();
    let start = Instant::now();
    quicksort(&mut arr);
    let duration: Duration = start.elapsed();

    println!("Time elapsed in quicksort() is: {:?}", duration);
}

fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

fn _quicksort<T: Ord>(arr: &mut [T], l: isize, r: isize) {
    if l <= r {
        let q = partition(arr, l, r);
        _quicksort(arr, l, (q - 1) as isize);
        _quicksort(arr, (q + 1) as isize, r);
    }
}

fn partition<T: Ord>(arr: &mut [T], l: isize, r: isize) -> isize {
    let mut q = l;
    // println!("p: {}\tr: {}\tq: {}", l, r, q);
    for u in l..r {
        if arr[u as usize] <= arr[r as usize] {
            arr.swap(q as usize, u as usize);
            q += 1;
        }
    }
    arr.swap(q as usize, r as usize);
    return q;
}

// fn printarr(arr: &[u32], str: &str) {
//     print!("{}", str);
//     for i in 0..arr.len() {
//         println!("{}", arr[i]);
//     }
//     println!();
// }
