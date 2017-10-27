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

#[test]
fn test_extract() {
    let mut a = unsorted(37*D+129);
    let mut b = a.clone();
    a.sort();
    heapify(&mut b);
    for i in (0..a.len()).rev() {
        let e = extract(&mut b);
        checkheap(&b, 0);
        assert!(a[i] == e);
    }
}

#[test]
fn test_insert() {
    let mut a = unsorted(37*D+129);
    let mut b = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        insert(&mut b, a[i]);
        checkheap(&b, 0);
    };
    a.sort();
    for i in (0..a.len()).rev() {
        let e = extract(&mut b);
        checkheap(&b, 0);
        assert!(a[i] == e);
    }
}
