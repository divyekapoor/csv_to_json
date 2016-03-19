use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::process::exit;

fn usage() {
  println!("Usage: cat input.csv | csv_to_json");
}

fn main() {
  let args : Vec<String> = env::args().collect();

  if args.len() > 1 {
    usage();
    exit(1);
  }

  let mut reader = BufReader::new(io::stdin());
  let split_char = ',';

  let mut line = String::new();
  let _unused_size = reader.read_line(&mut line);

  let headers : Vec<&str> = line.trim().split(split_char).collect();

  let mut first = true;

  for line in reader.lines() {
    if first {
      print!("[");
      first = false;
    } else {
      println!(",");
    }

    // Opening brace (escaped).
    print!("{{");

    let line_str = line.unwrap();

    let mut first = true;
    let components : Vec<&str> = line_str.split(split_char).collect();
    for (header, component) in headers.iter().zip(components.iter()) {
      if !first {
        print!(",");
      } else {
        first = false;
      }
      print!("\"{}\": \"{}\"", header, component);
    }

    // Closing brace (escaped).
    print!("}}");
  }
  println!("]");
}
