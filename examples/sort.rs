// Copyright © 2017 Bart Massey
// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// d-ary heapsort demo

extern crate dheap;

// XXX Copy-paste of test code.

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

pub fn main() {
    let mut a = unsorted(100);
    dheap::heapsort(&mut a);
    for e in a {
        println!("{}", e)
    }
}
