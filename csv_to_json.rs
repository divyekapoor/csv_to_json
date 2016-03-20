use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::process;
use std::fs::File;

fn usage() {
  println!("Usage: cat input.csv | csv_to_json");
  println!("Usage: csv_to_json input.csv");
}

fn main() {
  let stdin = io::stdin();
  let mut filereader;
  let mut reader = &mut stdin.lock() as &mut BufRead;
  let args : Vec<String> = env::args().collect();

  if args.len() > 2 {
    usage();
    process::exit(1);
  } else if args.len() == 2 {
    let file = File::open(args[1].clone());
    if !file.is_ok() {
      usage();
      process::exit(1);
    }
    filereader = BufReader::new(file.unwrap());
    reader = &mut filereader;
  }

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
