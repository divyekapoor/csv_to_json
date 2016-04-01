extern crate csv;

use std::io;
use std::io::prelude::*;
use std::env;
use std::process;
use std::fs::File;

fn usage() {
  println!("Usage: cat input.csv | csv_to_json");
  println!("Usage: csv_to_json input.csv");
}

fn arg_or_stdin(filename_or_none: Option<String>) -> Result<Box<Read>, io::Error> {
  match filename_or_none {
    Some(filename) => {
      let file = try!(File::open(filename));
      Ok(Box::new(file) as Box<Read>)
    },
    None => {
      Ok(Box::new(io::stdin()) as Box<Read>)
    }
  }
}

fn row_to_object(headers: &Vec<String>, row: &Vec<String>) -> Result<String, String> {
  if row.len() > headers.len() {
    return Err(format!("Size mismatch between headers ({}) and row ({})", headers.len(), row.len()));
  }

  let joined_fields = headers.iter().zip(row.iter()).map(|(header, row)|
    format!("\"{}\": \"{}\"", header, row)).collect::<Vec<String>>().join(",");

  Ok(
    ["{", &joined_fields[..],  "}"].concat()
  )
}

fn main() {
  let args : Vec<String> = env::args().collect();

  if args.len() > 2 {
    usage();
    process::exit(1);
  }


  let input = arg_or_stdin(env::args().nth(1));
  match input {
    Err(e) => {
      println!("Unable to open file: {:?}", e);
      usage();
      process::exit(1);
    },
    Ok(_) => {}
  };

  let mut csv_reader = csv::Reader::from_reader(input.unwrap()).has_headers(true);

  let headers : Vec<String> = csv_reader.headers().unwrap_or(Vec::new());

  print!("[");

  let mut first = true;
  for row in csv_reader.decode() {
    if !first {
      println!(",");
    } else {
      first = false;
    }
    let row_value : Vec<String> = row.unwrap_or(Vec::new());
    print!("{}", row_to_object(&headers, &row_value).unwrap_or("".to_owned()));
  }

  println!("]");
}
