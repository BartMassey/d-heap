// Copyright © 2017 Bart Massey
// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// d-ary max-heap with heapsort

// Benchmark sort timings, 1M in-order entries:
// D=8, 0.15s
// D=4, 0.12s
// D=2, 0.14s
pub const D: usize = 4;

fn downheap<E: Ord>(a: &mut[E], i0: usize) {
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

pub fn heapify<E: Ord>(a: &mut[E]) {
    for i in (0..a.len()).rev() {
        downheap(a, i)
    }
}

pub fn heapsort<E: Ord>(a: &mut[E]) {
    heapify(a);
    for i in (1..a.len()).rev() {
        a.swap(0, i);
        downheap(&mut a[0..i], 0)
    }
}

pub fn extract<E: Ord>(a: &mut Vec<E>) -> E {
    let n = a.len();
    assert!(n > 0);
    a.swap(0, n - 1);
    let r = a.pop().unwrap();
    if a.len() > 0 {
        downheap(a, 0)
    };
    r
}

fn upheap<E: Ord>(a: &mut[E], i0: usize) {
    let mut i = i0;
    while i > 0 {
        let p = (i - 1) / D;
        if a[p] >= a[i] {
            return
        }
        a.swap(p, i);
        i = p;
    }
}

pub fn insert<E: Ord>(mut a: &mut Vec<E>, v: E) {
    let i = a.len();
    a.push(v);
    upheap(&mut a, i)
}
