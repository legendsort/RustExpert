#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::f32::consts::PI;

use std::thread;
use std::time::Duration;

fn main() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Thread : {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    thread1.join().unwrap();

}

