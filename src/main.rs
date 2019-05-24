#![feature(reverse_bits)]
extern crate bitvec;
extern crate itertools;

use bitvec::{
    vec::BitVec,
    cursor::LittleEndian,
};

use std::iter::FromIterator;
use itertools::Either;

fn twos_complement(val: u32) -> u32 {
    !val + 1u32
}

fn bit_add(a: bool, b: bool, cin: bool) -> (/*s*/ bool, /*cout*/ bool) {
    let s = cin ^ ( a ^ b);
    let cout = (a & b) | ((a ^ b) & cin);
    (s, cout)
}

fn assemble_byte(mut v: Vec::<bool>) -> u8 {
    v.reverse();
    let mut output: u8 = 0;
    for bit in v {
        output = {
            let shift = output << 1;
            let bit = match bit {
                true => 1u8,
                false => 0u8,
            };
            shift | bit
        };
    }
    output
}

fn get_bytes(bits: &mut BitVec::<LittleEndian, u8>) -> [u8; 4] {
    let mut output: [u8; 4] = [0; 4];
    for i in 0..4 { 
        output[i] = assemble_byte(Vec::from_iter(bits.drain(0..8).into_iter()));
    }
    return output;
}

pub fn bin_add(x: u32, y: u32) -> u32 {

    let (xbits, ybits) = (BitVec::<LittleEndian, u32>::from_element(x), BitVec::<LittleEndian, u32>::from_element(y));

    let mut bits = BitVec::<LittleEndian, u8>::with_capacity(32);
    let mut carry: bool = false;
    for (xbit, ybit) in xbits
                                 .clone()
                                 .iter()
                                 .zip(ybits.clone().iter())
    {
        let (sum, cout) = bit_add(xbit, ybit, carry);
        bits.push(sum);
        carry = cout;
    }
    
    let output = u32::from_le_bytes(get_bytes(&mut bits));

    return output;    

}

pub fn bin_sub(x: u32, y: u32) -> Either<u32, i32> {
    if y > x {
        Either::Right(bin_add(x, twos_complement(y)) as i32)
    } else {
        Either::Left(bin_add(x, twos_complement(y)))
    }
}


pub fn bin_mul(x: u32, y: u32) -> u32 {
    let mut total: u32 = 0;
    for _ in 0..y {
        total = bin_add(total, x);
    }
    return total;
}


pub fn bin_div() {

}




fn main() {
    eprintln!("{}", bin_mul(10u32, 32u32));
    // let (x, y) = (11, 5);
    // for i in 0..100 {
    //     eprintln!("bin_add({}, {}) -> {}", x, i, bin_add(x, i));
    // }
} 