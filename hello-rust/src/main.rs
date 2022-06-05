fn main() {
    println!("GCD of {} and {} is {}",
             43,
             44,
             gcd(44, 43));
}

fn gcd(n: u32, m: u32) -> u32 {
    fn iter(small: u32, large: u32) -> u32 {
        match small {
            0 => large,
            x => iter(large % x, x),
        }
    }
    match n < m {
        true => iter(n, m),
        false => iter(m, n),
    }
}

