#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;


fn main() {
    let mut hs: HashMap<&str, &str> = HashMap::new();
    hs.insert("bu", "mo");
    hs.insert("do", "js");
    hs.insert("three", "js");

    for (k, v) in hs.iter() {
        println!("{}, {}", k, v);
    }

    if hs.contains_key(&"do") {
        let ans = hs.get(&"do");
        match ans {
            Some(x) => print!("Good"),
            None => print!("Bad"),
        }
    }

}
