use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("What's the name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";

    // save value to name
    io::stdin().read_line(&mut name)
        .expect("Didn't get your name");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
