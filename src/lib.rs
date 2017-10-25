// Copyright © 2017 Bart Massey
// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// d-ary max-heap of u32 with heapsort

// Benchmark sort timings, 1M in-order entries:
// D=8, 0.15s
// D=4, 0.12s
// D=2, 0.14s
pub const D: usize = 4;

fn downheap(a: &mut[u32], i0: usize) {
    let mut i = i0;
    loop {
        let mut m = i;
        for j in D*i+1..D*i+1+D {
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

pub fn heapify(a: &mut[u32]) {
    for i in (0..a.len()).rev() {
        downheap(a, i)
    }
}

pub fn heapsort(a: &mut[u32]) {
    heapify(a);
    for i in (1..a.len()).rev() {
        a.swap(0, i);
        downheap(&mut a[0..i], 0)
    }
}

pub fn extract(a: &mut Vec<u32>) -> u32 {
    assert!(a.len() > 0);
    let r = a[0];
    let m = a.pop().unwrap();
    if a.len() > 0 {
        a[0] = m;
        downheap(a, 0);
    };
    r
}

fn upheap(a: &mut[u32], i0: usize) {
    let mut i = i0;
    while i > 0 {
        let p = (i - 1) / D;
        let mut m = p;
        for j in D*p+1..D*p+1+D {
            if j >= a.len() {
                break
            };
            if a[j] > a[m] {
                m = j
            }
        };
        if m == p {
            return
        };
        a.swap(p, m);
        i = p;
    }
}

pub fn insert(mut a: &mut Vec<u32>, v: u32) {
    let i = a.len();
    a.push(v);
    upheap(&mut a, i)
}
