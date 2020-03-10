use std::io;
use std::process;

fn main() {
  println!("Enter first number");
  let mut first = String::new();
  io::stdin().read_line(&mut first).unwrap();

  let mut a:u32;
  match first.trim().parse() {
    Ok(val) => {
      a = val;
    },
    Err(_err) => {
      println!("Not a valid number.");
      process::exit(1);
    }
  }

  println!("Enter second number");
  let mut second = String::new();
  io::stdin().read_line(&mut second).unwrap();

  let mut b:u32;
  match second.trim().parse() {
    Ok(val) => {
      b = val;
    },
    Err(_err) => {
      println!("Not a valid number.");
      process::exit(1);
    }
  }

  let result = sum(a, b);
  println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
  return a + b;
}
