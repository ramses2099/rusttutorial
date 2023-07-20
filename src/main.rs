#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};

mod restaurant;
use crate::restaurant::order_food;
fn main() {
  order_food();


}
