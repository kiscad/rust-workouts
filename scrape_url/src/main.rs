use std::env;
use std::fs;

fn main() {
    // let url = "https://www.rust-lang.org/";
    // let output = "rust.md";
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Not enough args provide");
        eprintln!("Usage: cargo run -- URL-ADDR MD-FILE-PATH");
        std::process::exit(1);
    }
    let (url, output) = (&args[0], &args[1]);
    println!("url: {}", url);
    println!("output file: {}", output);

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)
        .expect("Invalid url")
        .text()
        .unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been save in {}", output);
}
