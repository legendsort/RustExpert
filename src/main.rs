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
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle { length, width }
        }
        fn area(&self) -> f32 {
            return self.width * self.length;
        }
    }
    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width }
        }
        fn area(&self) -> f32 {
            return self.width * self.length * PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0, 10.0);
    let cir: Circle = Shape::new(10.0, 10.0);
    print!("rec: {}", rec.area());
    print!("cir: {}", cir.area());
    

}
