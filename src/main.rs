extern crate rand;
extern crate num;
extern crate core;
extern crate time;

use std::time::{Duration};
use time::{now, Tm};

use rand::Rng;
use num::{Zero, One};
use num::bigint::{RandBigInt, BigUint, ToBigUint};
use num::traits::{ToPrimitive};
use num::integer::Integer;
use core::ops::{Add, Sub, Mul, Div, Rem, Shr};

fn find_r_and_d(i: BigUint) -> (u64, BigUint) {
    let mut d = i;
    let mut r = 0;
    loop {
        if d.clone().rem(&2u64.to_biguint().unwrap()) == Zero::zero() {
            d = d.shr(1usize);
            r = r + 1;
        } else {
            break;
        }
    }
    return (r, d);
}

fn might_be_prime(n: &BigUint) -> bool {
    let one = BigUint::one();
    let nsub1 = n - &one;
    let two = BigUint::new(vec![2]);
    let mut rng = rand::thread_rng();

    let (r, mut d) = find_r_and_d(nsub1.clone());
    let mut x;
    let mut a: BigUint;
    'WitnessLoop: for kk in 0..6u64 {
        a = rng.gen_biguint_range(&two, &nsub1);
        x = mod_exp(&mut a, &mut d, &n);
        if &x == &one || x == nsub1 {
            continue;
        }
        for rr in 1..r {
            x = (&x * &x) % n;
            if &x == &one {
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

    while &*exponent > &zero {
        if &*exponent & &one == one {
           result = (result * &*base) % modulus;
        }
        *base = (&*base * &*base) % modulus;
        *exponent = &*exponent >> 1usize;
    }
    result
}

fn main() {  
    let now1 = now();

    let mut b = "3644156651806149735145554752510739718312305395535692520361243243621494715240964000096007396464293664302720098339200581682956734880917676321024312391836481599463".parse::<BigUint>().unwrap();
    while true {
        let one = BigUint::one();
        b = b + (&one + &one);
        if might_be_prime(&b) {
            println!("{}", b);
        }
    }

    let now2 = now();
    println!("{}", now2.to_timespec().sec - now1.to_timespec().sec);
}  