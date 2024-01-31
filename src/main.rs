#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
}

fn loops() {
    let arr_1 = [1, 2, 3, 4];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;
    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    println!("while loop");
    let mut idx = 0;
    while idx < arr_2.len() {
        println!("Val : {}", arr_2[idx]);
        idx += 1;
    }

    println!("for loop");
    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}

fn match_function() {
    let age = 8;
    if age >= 1 && age <= 18 {
        println!("Important birthday");
    } else if (age == 21 || age == 50) {
        println!("A bit important");
    } else if (age > 65) {
        println!("I guess also important birthday");
    } else {
        println!("Not important");
    }

    let can_vote = if age >= 18 {true} else {false};
    println!("Can vote? {}", can_vote);

    let new_age = 47;
    match new_age {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("Somewhat important birthday"),
        65..=i32::MAX => println!("ig also importatnt"),
        _ => println!("Not important"),
    }

    let my_age = 68;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("CAN vote"),
        Ordering::Equal => println!("Just got the right"),
    }
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
