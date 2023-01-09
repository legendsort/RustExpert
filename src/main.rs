#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::f32::consts::PI;

fn main() {
    let mut arr = [1, 2, 3, 4];
    for val in arr.iter() {
        println!("{}", val);
    }
    let mut iter1 = arr.iter();
    println!("{:?}", iter1.next());


    let can = |age: i32| {
        age >= 18
    };

    println!("Can vote: {}", can(8));

    let mut samp1 = 5;
    let var = || println!("{}", samp1);
    var();
    samp1 = 5;
    let mut change = || samp1 += 1;
    change();
    println!("{}", samp1);



    fn use_func<T>(a: i32, b: i32, func: T) -> i32 
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }
    let sum = |a, b| a +b;
    let pro = |a, b| a*b;
    println!("{}", use_func(5, 4, sum));
    println!("{}", use_func(5, 4, pro));
}

