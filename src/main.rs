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
    let nsub1 = n.sub(1u64.to_biguint().unwrap());
    let two = 2u64.to_biguint().unwrap();

    let (r, d) = find_r_and_d(nsub1.clone());
    'WitnessLoop: for kk in 0..6u64 {
        let a = rand::thread_rng().gen_biguint_range(&two, &nsub1);
        let mut x = mod_exp(&a, &d, &n);
        if x == 1u64.to_biguint().unwrap() || x == nsub1 {
            continue;
        }
        for rr in 1..r {
            x = x.clone().mul(x.clone()).rem(n);
            if x == 1u64.to_biguint().unwrap() {
                return false;
            } else if x == nsub1 {
                continue 'WitnessLoop;
            } 
        }
        return false;
    }
    return true;
}

fn mod_exp(base: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    let one = 1u64.to_biguint().unwrap();
    let mut result = one.clone();
    let mut base_clone = base.clone();
    let mut exponent_clone = exponent.clone();

    while exponent_clone > 0u64.to_biguint().unwrap() {
        if exponent_clone.clone() & one.clone() == one {
            result = result.mul(&base_clone).rem(modulus);
        } 
        base_clone = base_clone.clone().mul(base_clone).rem(modulus);
        exponent_clone = exponent_clone.shr(1usize);
    }
    return result;
}

fn main() {  
    let now1 = now();

    for n in 774541u64..18446744073709551615u64 {
        let b = n.to_biguint().unwrap();
        if might_be_prime(&b) {
            println!("{}", n);
        }
    }

    let now2 = now();
    println!("{}", now2.to_timespec().sec - now1.to_timespec().sec);
}  