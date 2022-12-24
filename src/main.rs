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

}
