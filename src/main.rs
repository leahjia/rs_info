#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
}

fn math_operators() {
    let is_true = true;
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_num);
}

fn data_types() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "67";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    println!("I'm {} and I want ${}", age, ONE_MIL);

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}

fn read_input() {
    println!("What's the name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";

    // save value to name
    io::stdin().read_line(&mut name)
        .expect("Didn't get your name");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
