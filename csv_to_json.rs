use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
  let split_char = ',';
  let stdin = io::stdin();
  let mut reader = BufReader::new(stdin);

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
