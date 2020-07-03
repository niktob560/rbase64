use std::io::{self, BufRead};
use std::string::String;


fn unfold_bits (from: Vec<u8>) -> Vec<u8> {
    let mut i = 0u8;
    let mut buf = 0u8;
    let mut ret = Vec::<u8>::new();
    for bit in from {
        if i < 6 {
            buf <<= 1;
            buf |= bit;
            i += 1;
        }
        if i >= 6 {
            i = 0;
            ret.push(buf);
            buf = 0;
        }
    }
    if buf != 0 {
        while i < 6 {
            buf <<= 1;
            i += 1
        }
        ret.push(buf)
    }
    ret
}

fn text_to_bit_vec(text: String) -> Vec<u8> {
    text.chars().fold(Vec::new(), |acc, x| {
        let mut a = acc; 
        let buf = x as u8;
        for i in 0..8 {
            a.push((buf >> (7 - i)) & 1);
        }
        a
    })
}

fn bit_vec_to_encoded_text(vec: Vec<u8>) -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string();
    vec.iter().fold("".to_string(), |acc, x| {
        let mut a = acc;
        a.push(alphabet.chars().nth(*x as usize).unwrap());
        a
    })
}

fn base64 (from: String) -> String {
    let mut ret = bit_vec_to_encoded_text(unfold_bits(text_to_bit_vec(from)));
    while ret.len() % 4 != 0 {
        ret.push('=')
    }
    ret
}

fn main() {
    // let stdin = io::stdin();
    // for lines in stdin.lock().lines() {

    // }
    println!("{}", base64("Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure.".to_string()));
}
