use std::io;

fn main() {
  println!("Enter first number");
  let mut first = String::new();
  io::stdin().read_line(&mut first);

  let a:u32 = first.trim().parse().expect("Not a valid number.");

  println!("Enter second number");
  let mut second = String::new();
  io::stdin().read_line(&mut second);

  let b:u32 = second.trim().parse().expect("Not a valid number.");

  let result = sum(a, b);
  println!("{} + {} = {}", a, b, result);
}

fn sum(a: u32, b: u32) -> u32 {
  return a + b;
}
