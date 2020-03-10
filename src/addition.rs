use std::io;
use std::process;

fn main() {
  loop {
    println!("Enter first number");
    let a = read_user_input_as_number();

    println!("Enter second number");
    let b = read_user_input_as_number();

    let result = sum(a, b);
    println!("{} + {} = {}", a, b, result);
  }
}

fn sum(a: u32, b: u32) -> u32 {
  return a + b;
}

fn read_user_input_as_number() -> u32 {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let digit:u32;
  match input.trim().parse() {
    Ok(val) => digit = val,
    Err(_err) => {
      println!("Not a valid number.");
      process::exit(1);
    }
  }
  return digit;
}
