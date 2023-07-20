#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};

//struct
struct Customer {
    name: String,
    address: String,
    balance: f32,
}
fn main() {
    //instant
    let mut bob = Customer {
        name: String::from("bob Perez"),
        address: String::from("23 los rios"),
        balance: 5.00,
    };

}
