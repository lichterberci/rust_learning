use std::{
    cmp::Ordering,
    ops::{Rem, Sub},
};

pub use num_bigint::BigUint;
use rand::Rng;

pub fn get_rand_prime(num_bits: u32) -> Option<BigUint> {
    let mut rng = rand::thread_rng();

    'outer: for _ in 0..100000 {
        let bytes = (0..num_bits.div_ceil(8))
            .map(|_| rng.gen::<u8>())
            .collect::<Vec<u8>>();

        let modulus = BigUint::from(2u8).pow(num_bits);

        let n = BigUint::from_bytes_be(&bytes).rem(&modulus);

        let one = BigUint::from(1u8);

        if (&n).cmp(&BigUint::from(2u8)) == Ordering::Less {
            continue;
        }

        for _ in 0..100 {
            let bytes = (0..num_bits.div_ceil(8))
                .map(|_| rng.gen::<u8>())
                .collect::<Vec<u8>>();

            let a = BigUint::from_bytes_be(&bytes).rem(&(&n).sub(BigUint::from(1u8)));

            if a.cmp(&BigUint::from(2u8)) == Ordering::Less {
                continue 'outer;
            }

            let a_to_n_minus_one = (&a).modpow(&(&n).sub(&one), &n);

            if a_to_n_minus_one.eq(&one) == false {
                continue 'outer;
            }
        }

        return Some(n);
    }

    None
}
