#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::cmp::Ordering;
use modulo_n_tools::*;

fn crc (mut byte_to_encode: u16) -> u16 {

    if byte_to_encode == 0 {
        return byte_to_encode;
    }

    let mut gen_polynom: u16 = 0b101001;
    let n_plus_1 = gen_polynom.count_ones() + gen_polynom.count_zeros() - gen_polynom.leading_zeros();
    byte_to_encode <<= n_plus_1 - 1;

    // println!("The generator polynomial, v15: {:b}", gen_polynom);
    // println!("{}th degree polynomial", n_plus_1 - 1);
    // println!("Padded input: {:b}", byte_to_encode);

    let len_pad = byte_to_encode.count_ones() + byte_to_encode.count_zeros() - byte_to_encode.leading_zeros();

    gen_polynom <<= len_pad - n_plus_1;

    let mut shifted_pol = gen_polynom;

    let unnes = 16 - len_pad;

    while byte_to_encode.count_ones() + byte_to_encode.count_zeros() - byte_to_encode.leading_zeros() > n_plus_1 - 1 {
        byte_to_encode ^= shifted_pol;
        let curr_shift = byte_to_encode.leading_zeros() - unnes;
        shifted_pol = gen_polynom >> curr_shift;
    }

    byte_to_encode
}

fn collision () -> HashMap<u16, i32> {
    let mut collisions = HashMap::new();

    for n in 0..256 {

        match collisions.get_mut(&crc(n)) {
            Some(count) => {
                *count += 1;
            },
            None => {
                collisions.insert(crc(n), 1);
            }
        }

    }
    collisions
}

#[cfg(test)]
mod crc_test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(0b01101, crc(0b1000_1001));
    }
    #[test]
    fn test2() {
        assert_eq!(0b11010, crc(0b1000));
    }
    #[test]
    fn test3() {
        assert_eq!(0b01101, crc(0b100));
    }
    #[test]
    fn test4() {
        assert_eq!(0b11110, crc(0b01101));
    }
    #[test]
    fn test5() {
        assert_eq!(0b0, crc(0b101001));
    }
    #[test]
    fn test6() {
        assert_eq!(0b0, crc(0b0));
    }
}

#[cfg(test)]
mod collisions_test {
    use super::*;

    #[test]
    fn dict_size() {
        assert_eq!(32, collision().len());
    }

    #[test]
    fn dict_sum() {
        let s: Vec<i32> = collision().into_values().collect();
        assert_eq!(256, s.iter().sum::<i32>());
    }
}

pub fn gcd<T>(a: T, b: T) -> T
    where
        T: PartialEq + std::ops::Rem<Output = T> + Default + Copy,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

fn elgamal (M: u16, verbosity: bool, p: i32, g: i32) -> (i32, i32, i32) {
    // по варианту
    let x = 5;

    let y = pow_mod(g, x, &p);

    let h = crc(M);

    let result = h.cmp(&(p as u16));
    assert_eq!(Ordering::Less, result);

    let mut k= 0;

    for i in 2..p-1 {
        if gcd(i, p-1) == 1 {
            k = i;
            break;
        }
    }

    let r = pow_mod(g, k, &p);

    let mut u = sub_mod(&(h as i32), &(x * r), &(p-1)); // -64 + 162 = 98
    if u < 0 {
        u += p - 1;
    }

    let mut k_inv = 0;
    for i in 0 .. p-1 {
        if k * i % (p-1) == 1 {
            k_inv = i;
            break;
        }
    }

    let s = mul_mod(&k_inv, &u, &(p-1)); // -110 + 162 = 52

    if verbosity {
        println!("Open key y = {}", y);
        println!("r = {}^{} mod {} = {}", g, k , p , r);
        println!("u = ( {} - {} * {} ) mod {} = {} ", h, x, r, p-1, u);
        println!("s = {} * {} mod {} = {}", k_inv, u, p-1, s);
        println!("Signed (M, r, s): ({}, {}, {})", M, r, s);
    }

    (r, s, y)
}

fn elgamal_check(M: u16, r: i32, s: i32, y: i32, p: i32, g: i32) -> bool {
    let h_ch = crc(M);

    let right = pow_mod(g, h_ch, &p);
    let left = mul_mod(&pow_mod(y, r, &p), &pow_mod(r, s, &p), &p);

    left == right
}

fn main() {
    let M = 0b1000;

    println!("CRC for message {:b} is {:05b} with the generator polynomial, v15: {:b}", M, crc(M), 0b101001);

    println!("Let's take a look at the collision statistics!");

    for (crc, count) in &collision() {
        println!("{:05b} was there {} times", crc, count);
    }

    println!("\nNow it's time to sign the same message {} using ElGamal signature scheme", M);

    let p = 163;
    let g = 7;

    println!("p, g, x for v15 are {}, {}, 5", p, g);

    let (r, s, y) = elgamal(M, true, p, g);

    println!("Is the message {} signed with r = {}? {}", M, r, elgamal_check(M, r, s, y, p, g));

    println!("Is the message {} signed with r = {}? {}", M, 17, elgamal_check(M, 0b10011, s, y, p, g));
}
