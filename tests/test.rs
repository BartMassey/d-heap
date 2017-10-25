// Copyright © 2017 Bart Massey
// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// tests for d-ary heap

extern crate dheap;
use dheap::*;

extern crate pcg_rand;
extern crate rand;

use rand::Rng;
use pcg_rand::Pcg32;

fn unsorted(n: usize) -> Vec<u32> {
    let mut pcg = Pcg32::new_unseeded();
    let mut a: Vec<u32> = Vec::with_capacity(n);
    // XXX First call always returns 0. This seems like a bug.
    let _: u32 = pcg.gen();
    for _ in 0..n {
        a.push(pcg.gen())
    };
    a
}

fn checkheap(a: &[u32], i: usize) {
    for j in D*i+1..D*i+1+D {
        if j >= a.len() {
            return
        };
        assert!(a[j] <= a[i]);
        checkheap(a, j)
    }
}

#[test]
fn test_heapify() {
    let mut a = unsorted(3*D+17);
    heapify(&mut a);
    checkheap(&a, 0)
}

#[test]
fn test_heapsort() {
    let mut a = unsorted(3*D+17);
    heapsort(&mut a);
    for i in 1..a.len() {
        assert!(a[i-1] <= a[i]);
    }
}
