use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = vec![];
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).unwrap());
    }
    if numbers.len() == 0 {
        eprintln!("Usage: cargo run NUMBERS ...");
        std::process::exit(1);
    }
    let res = numbers.iter().skip(1)
        .fold(numbers[0], |a, b| gcd(a, *b));
    println!("GCD of {:?} is {}", &numbers, res);
}

fn gcd(n: u64, m: u64) -> u64 {
    fn iter(sm: u64, bg: u64) -> u64 {
        match sm {
            0 => bg,
            _ => iter(bg % sm, sm),
        }
    }
    if n < m {
        iter(n, m)
    } else {
        iter(m, n)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
