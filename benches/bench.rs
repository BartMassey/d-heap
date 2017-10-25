// Copyright © 2017 Bart Massey
// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// benches for d-ary heap

extern crate dheap;
use dheap::*;

extern crate pcg_rand;
extern crate rand;
extern crate criterion;
use criterion::Criterion;

use rand::Rng;
use pcg_rand::Pcg32;

const K: usize = 1024;

const N_BENCH: u32 = 16*K as u32;

// XXX Copy-paste from test.
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


#[test]
fn bench_heapsort() {
    Criterion::default().bench_function_over_inputs("heapsort",
        |b, &&size| {
            let mut a: Vec<u32> = (0u32..size as u32).collect();
            b.iter(|| heapsort(&mut a))
        }, &[K, 4*K, 16*K, 64*K]);
}

fn heapsort_extract(mut a: &mut Vec<u32>) -> Vec<u32> {
    heapify(a);
    let mut r = Vec::with_capacity(a.len());
    while a.len() > 0 {
        let v = extract(&mut a);
        r.push(v);
    };
    r.reverse();
    r
}

#[test]
fn test_heapsort_extract() {
    let n = 3*D+17;
    let mut a = unsorted(n);
    let r = heapsort_extract(&mut a);
    assert!(r.len() == n);
    for i in 1..n {
        assert!(r[i-1] <= r[i]);
    }
}

#[test]
fn bench_heapsort_extract() {
    let a: Vec<u32> = (0..N_BENCH).collect();
    Criterion::default().bench_function(
        "heapsort_extract",
        |b| b.iter(|| {
            let r = heapsort_extract(&mut a.clone());
            assert!(r[0] == 0)}));
}


fn heapsort_insert(a: &mut Vec<u32>) {
    let n = a.len();
    let mut t = Vec::with_capacity(n);
    while a.len() > 0 {
        let v = a.pop().unwrap();
        insert(&mut t, v);
    };
    for _ in 0..n {
        let v = extract(&mut t);
        a.push(v);
    };
    a.reverse()
}

#[test]
fn test_heapsort_insert() {
    let n = 3*D+17;
    let mut a = unsorted(n);
    heapsort_insert(&mut a);
    assert!(a.len() == n);
    for i in 1..n {
        assert!(a[i-1] <= a[i]);
    }
}

#[test]
fn bench_heapsort_insert() {
    let mut a: Vec<u32> = (0..N_BENCH).collect();
    Criterion::default().bench_function(
        "heapsort_insert", |b| b.iter(|| heapsort_insert(&mut a)));
}
