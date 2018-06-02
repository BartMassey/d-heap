// Copyright © 2017 Bart Massey
// This work is made available under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

// d-ary heapsort demo

extern crate dheap;

// XXX Copy-paste of test code.

extern crate rand;

use rand::Rng;

fn unsorted(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut a: Vec<u32> = Vec::with_capacity(n);
    let _: u32 = rng.gen();
    for _ in 0..n {
        a.push(rng.gen())
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
