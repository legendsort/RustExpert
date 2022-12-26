#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    let mut my_age = 47;
    let voting_age = 18;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("{}", can_vote);

    let age2 = 8;
    match age2  {
        1..=18 => println!("Imp"),
        21 | 50 => println!("Imp"),
        65..=i32::MAX => println!("Imp"),
        _ => println!("Not imp")
    };

   match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can't vote"),
        Ordering::Equal => println!("Can't vote"),
        
   } 

   let arr = [1, 2, 3, 4, 5];
   let mut loop_id = 0;
   loop {
    if arr[loop_id] % 2 == 0 {
        loop_id += 1;
        continue
    }
    if arr[loop_id] == 5 {
        break;
    }
    println!("val : {}", arr[loop_id]);
    loop_id += 1;
   }

   for val in arr.iter() {
    println!("val: {}", val);
   }

   let tup: (u8, String, f64) = (47, "DO".to_string(), 50_000.0);
   println!("NAME:{}", tup.1);
   let (v1, v2, v3) = tup;
   println!("Age:{}", v1);
}
