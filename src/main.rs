#![allow(unused)]

use core::f32;
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
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

fn main() {
    pub struct Bank {
        balance: f32
    }
    fn withdraw(bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bankref = bank.lock().unwrap();
        if bankref.balance < 5.00 {
            println!("CAn't {}", bankref.balance);
        } else {
            bankref.balance -= amt;
            println!("Withdrawed {} {}", amt, bankref.balance);
        }
    }

    fn customer(bank: &Arc<Mutex<Bank>>) {
        withdraw(&bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> = Arc::new(Mutex::new(Bank {balance: 20.00}));
    let handles = (0..10).map(|_| {
        let bankref = bank.clone();
        thread::spawn(move || {
            customer(&bankref)
        })
    });
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);


}

