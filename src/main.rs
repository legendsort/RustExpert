#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add;
use std::collections::HashMap;
use std::f32::consts::PI;

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    let lil_arr = [1, 2, 3];
    println!("{}", lil_arr[10]);

}
