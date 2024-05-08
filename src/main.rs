#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::Add;

fn main() {
}

fn print_stuff() {
    let mut str1 = String::from("Leah");
    let str2 = str1.clone();
    println!("Hwllo {}", str1);

    // this will block the next call 
    // print_str(str1);

    // this will block the next call
    // let str3 = print_return_str(str1);
    // println!("str3 = {}", str3);

    change_string(&mut str1);
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Msg: {}", name);
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn get_sum(x: i32, y: i32) -> i32 {
    println!("{} + {} = {}", x, y, x + y);
    x + y
}

fn vectors() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 2, 3, 4, 5];
    vec2.push(6);
    println!("1st : {}", vec2[0]);

    // verify vector exists
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value"),
    }
    // cycle through vector; change val
    for i in &mut vec2 {
        // manipulate value
        *i *= 2;
    }
    // verify changes
    for i in &mut vec2 {
        println!("Val: {}", i);
    }
    println!("Vec Length = {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}

fn numerated_types() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;
    match today {
        Day::Monday => println!("Worst day ever"),
        Day::Tuesday => println!("Chill day"),
        Day::Wednesday => println!("Meeting day"),
        Day::Thursday => println!("Almost Friday"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Sunday Funday"),
    }

    println!("Is today the weekend? {}", today.is_weekend());
}

fn strings() {
    let mut str = String::new();
    str.push('A');
    str.push_str(" word");

    for word in str.split_whitespace() {
        println!("{}", word);
    }

    let str2 = str.replace("A", "Another");
    println!("{}", str2);

    let str3 = String::from("a d g t gg  t e w");
    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    // create string literal
    let str4: &str = "Random string";
    // convert to heap allocated string
    let mut str5: String = str4.to_string();
    println!("{}", str5);

    let byte_arr = str5.as_bytes();
    // get slice of a string
    let str6 = &str5[0..=5];
    println!("String length: {}", str6.len());
    str5.clear();

    // combine strings
    let str6 = String::from("Some other string");
    let str7 = String::from(" rando words lalala");
    let str8 = str6 + &str7;
    for ch in str8.chars() {
        print!("{} ", ch);
    }
}

fn tuples() {
    let some_tuple: (u8, String, f64) = (68, "Dawg".to_string(), 68_000.00);
    println!("Name : {}", some_tuple.1);

    let (v1, v2, v3) = some_tuple;
    println!("Age : {}", v1);
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

    let can_vote = if age >= 18 { true } else { false };
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
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't get your name");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
