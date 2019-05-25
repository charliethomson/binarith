extern crate bitvec;

use bitvec::{
    vec::BitVec,
    cursor::LittleEndian,
};

use std::iter::Iterator;

/*
returns the two's complement of `val`
*/
fn twos_complement(val: u32) -> u32 {
    !val + 1u32
}

/*
bit_add(a, b, carry in) -> (sum, carry out);
*/
fn bit_add(a: bool, b: bool, cin: bool) -> (/*s*/ bool, /*cout*/ bool) {
    let s = cin ^ ( a ^ b);
    let cout = (a & b) | ((a ^ b) & cin);
    (s, cout)
}

/*
gets a four byte array representing a u32
*/
fn get_bytes(bits: &mut BitVec::<LittleEndian, u8>) -> [u8; 4] {
    // eprintln!("{}", bits);
    let mut output: [u8; 4] = [0; 4];
    let bytes = bits.clone().into_vec();
    for i in 0..4 { 
        output[i] = bytes[i];
    }
    return output;
}

/*
bin_add(x, y) -> sum(x + y)
*/
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

/*
bin_sub(x, y) -> difference(x - y)
*/
pub fn bin_sub(x: u32, y: u32) -> Result<u32, i32> {
    if y > x {
        Err(bin_add(x, twos_complement(y)) as i32)
    } else {
        Ok(bin_add(x, twos_complement(y)))
    }
}

/*
bin_mul(x, y) -> product(x * y)
*/
pub fn bin_mul(mut x: u32, mut y: u32) -> u32 {
    let mut product: u32 = 0;
    while y != 0 {
        if (y & 1u32) == 1 { product = bin_add(x, product) }
        x <<= 1u32;
        y >>= 1u32;
    }
    return product;
}

/*
bin_div(numerator, denominator) -> (quotient, remainder)
*/
pub fn bin_div(num: u32, div: u32) -> (/*Quotient*/ u32, /*Remainder*/ u32) {

    let mut quotient = 0;
    let remainder;
    let mut current = num;
    loop {
        if current < div {
            remainder = current;
            break;
        } else {
            quotient += 1;
            current -= div;
        }
    }

    return (quotient, remainder);
}

