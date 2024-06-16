use rand::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    const SIZE: usize = 1000;
    let mut arr: Vec<u32> = (0..SIZE)
        .map(|_| rand::thread_rng().gen_range(1..SIZE as u32))
        .collect();
    let start = Instant::now();
    quicksort(&mut arr);
    let duration: Duration = start.elapsed();

    println!("Time elapsed in quicksort() is: {:?}", duration);
    printarr(&arr, "Sorted array: ");
}

fn quicksort<T: Ord>(arr: &mut [T]) {
    _quicksort(arr, 0, (arr.len() - 1) as isize);
}

fn _quicksort<T: Ord>(arr: &mut [T], l: isize, r: isize) {
    if l <= r {
        let q = partition(arr, l, r);
        _quicksort(arr, l, q - 1);
        _quicksort(arr, q + 1, r);
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
    q
}

fn printarr(arr: &[u32], str: &str) {
    println!("{}", str);
    for i in arr {
        println!("{}", i);
    }
    println!();
}
