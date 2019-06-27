//Original code written in
//“Programming Rust by Jim Blandy and Jason Orendorff (O’Reilly).
//Copyright 2018 Jim Blandy and Jason Orendorff, 978-1-491-92728-1.”
//Then modified by Paul Hubbard

use std::io::Write;
use std::str::FromStr;

//Get the greatest common denom
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
//This is a test function that tests the gcd function above
//run it by executing 'cargo test'
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);

    assert_eq!(gcd(2, 2), 2);
}

fn main() {
    let mut numbers = Vec::new(); //create a new vector

    //get the user input
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    //check that the input is greater than zero
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    //get the gcd
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
