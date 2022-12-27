#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let mut str = String::new();
    str.push('A');
    str.push_str(" Word");
    for word in str.split_whitespace() {
        println!("{}", word);
    }

    let st2 = str.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4 = "Random string";
    let mut st5 = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("{}", st6.len());
    st5.clear();


    let st6 = String::from("Just some");
    let st7 = String::from("ss");
    let st8 = st6 + &st7;
    println!("{}", st8);

    // use as for cast

    enum Days {
        Monday,
        Saturday,
        Sunday
    }

    impl Days {
        fn is_weekend(&self) -> bool {
            match self {
                Days::Saturday | Days::Sunday => true,
                _ => false
            }

        }
    }


    let vec1: Vec<i32> = Vec :: new();
    let mut vec2 = vec![1, 2, 3, 4];
    vec2.push(5);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("{}", second),
        None => print!("SDFDF"),
    }

    for i in &mut vec2 {
        *i *= 2;        
    }

    for i in &vec2{
        println!("{}", i);
    }
    println!("Vec{}", vec2.len());
    println!("Pop : {:?}", vec2.pop());

}
