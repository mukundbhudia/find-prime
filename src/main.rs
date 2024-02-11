extern crate core;
extern crate num;
extern crate rand;
extern crate time;

use time::now;

use core::ops::{Rem, Shr};
use num::bigint::{BigUint, RandBigInt, ToBigUint};
use num::{One, Zero};

fn find_r_and_d(i: BigUint) -> (u64, BigUint) {
    let mut d = i;
    let mut r = 0;
    loop {
        if d.clone().rem(&2u64.to_biguint().unwrap()) == Zero::zero() {
            d = d.shr(1usize);
            r += 1;
        } else {
            break;
        }
    }
    (r, d)
}

fn might_be_prime(n: &BigUint) -> bool {
    let one = BigUint::one();
    let nsub1 = n - &one;
    let two = BigUint::new(vec![2]);
    let mut rng = rand::thread_rng();

    let (r, mut d) = find_r_and_d(nsub1.clone());
    let mut x;
    let mut a: BigUint;
    'WitnessLoop: for _kk in 0..6u64 {
        a = rng.gen_biguint_range(&two, &nsub1);
        x = mod_exp(&mut a, &mut d, n);
        if x == one || x == nsub1 {
            continue;
        }
        for _rr in 1..r {
            x = (&x * &x) % n;
            if x == one {
                return false;
            } else if x == nsub1 {
                continue 'WitnessLoop;
            }
        }
        return false;
    }
    true
}

fn mod_exp(base: &mut BigUint, exponent: &mut BigUint, modulus: &BigUint) -> BigUint {
    let one = BigUint::one();
    let zero = BigUint::zero();
    let mut result = BigUint::one();

    while *exponent > zero {
        if &*exponent & &one == one {
            result = (result * &*base) % modulus;
        }
        *base = (&*base * &*base) % modulus;
        *exponent = &*exponent >> 1usize;
    }
    result
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: rusty-prime START MAX_PRIMES_TO_FIND");
        eprintln!(
            "Example: {} 17 5. Will find 5 more primes after 17.",
            args[0]
        );
        std::process::exit(1);
    }

    let start_prime = &args[1];
    let max_primes_to_find = &args[2];

    if start_prime == "1" {
        eprintln!("Please use a larger start prime than {}.", start_prime);
        std::process::exit(1);
    }

    println!(
        "Starting with {} and finding {} more primes.",
        start_prime, max_primes_to_find
    );

    let now1 = now();
    let mut i = 0;

    if let Ok(mut b) = start_prime.parse::<BigUint>() {
        if let Ok(max_primes_to_find) = max_primes_to_find.parse::<i32>() {
            while i < max_primes_to_find {
                let one = BigUint::one();
                b += &one + &one;
                if might_be_prime(&b) {
                    i += 1;
                    println!("{}", b);
                }
            }
        } else {
            println!("Maximum prime to find must be a valid integer");
        }
    } else {
        println!("Start prime must be a valid integer");
    }

    let now2 = now();
    println!(
        "Found {} primes in {} seconds.",
        i,
        now2.to_timespec().sec - now1.to_timespec().sec
    );
}
