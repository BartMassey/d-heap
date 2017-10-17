// Copyright © 2017 Bart Massey

// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// k-ary heapsort

#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

#[cfg(test)]
use test::Bencher;

extern crate pcg_rand;
extern crate rand;

use rand::Rng;
use pcg_rand::Pcg32;

// Benchmark sort timings, 1M in-order entries:
// K=8, 0.15s
// K=4, 0.12s
// K=2, 0.14s
const K: usize = 2;

fn unsorted(n: usize) -> Vec<u32> {
    let mut pcg = Pcg32::new_unseeded();
    let mut a: Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(pcg.gen())
    };
    a
}

fn downheap(a: &mut [u32], i0: usize) {
    let mut i = i0;
    loop {
        let mut m = i;
        for j in K*i+1..K*i+1+K {
            if j >= a.len() {
                break
            };
            if a[j] > a[m] {
                m = j
            };
        };
        if m == i {
            return
        };
        a.swap(i, m);
        i = m;
    }
}

fn heapify(a: &mut [u32]) {
    for i in (0..a.len()).rev() {
        downheap(a, i)
    }
}

#[cfg(test)]
fn checkheap(a: &[u32], i: usize) {
    for j in K*i+1..K*i+1+K {
        if j >= a.len() {
            return
        };
        assert!(a[j] <= a[i]);
        checkheap(a, j)
    }
}

#[test]
fn test_heapify() {
    let mut a = unsorted(3*K+17);
    heapify(&mut a);
    checkheap(&a, 0)
}

fn heapsort(a: &mut [u32]) {
    heapify(a);
    for i in (1..a.len()).rev() {
        a.swap(0, i);
        downheap(&mut a[0..i], 0)
    }
}

#[test]
fn test_heapsort() {
    let mut a = unsorted(3*K+17);
    heapsort(&mut a);
    for i in 1..a.len() {
        assert!(a[i-1] <= a[i]);
    }
}

#[cfg(test)]
#[bench]
fn bench_heapsort(b: &mut Bencher) {
    let mut a: Vec<u32> = (0..1024*1024).collect();
    b.iter(|| heapsort(&mut a))
}

pub fn main() {
    let mut a = unsorted(1024*1024);
    heapsort(&mut a);
    for i in 1..a.len() {
        println!("{}", a[i])
    }
}
