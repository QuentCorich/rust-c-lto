use std::time::Instant;

pub mod cbind;


fn add_numbers(x: i32, y: i32) -> i32 {
  x + y
}

fn main() {
  println!("Start");

  let rust_timer = Instant::now();
  let mut rust_sum = 0;
  for _i in 0..1000000 {
    rust_sum = add_numbers(rust_sum, 1);
  }
  let rust_time = i128::try_from(rust_timer.elapsed().as_nanos()).unwrap();

  let rustandc_timer = Instant::now();
  let mut rustandc_sum = 0;
  for _i in 0..1000000 {
    rustandc_sum = unsafe {
        cbind::add_numbers(rustandc_sum, 1)
    }
  }
  let rustandc_time = i128::try_from(rustandc_timer.elapsed().as_nanos()).unwrap();

  println!("Rust loop time: {}ns Rust and C loop time: {}ns", rust_time, rustandc_time);
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::Instant;

  fn add_numbers(x: i32, y: i32) -> i32 {
      x + y
  }

  #[test]
  fn lto_test() {
    let rust_timer = Instant::now();
    let mut rust_sum = 0;
    for _i in 0..1000000 {
      rust_sum = add_numbers(rust_sum, 1);
    }
    let rust_time = i128::try_from(rust_timer.elapsed().as_nanos()).unwrap();

    let rustandc_timer = Instant::now();
    let mut rustandc_sum = 0;
    for _i in 0..1000000 {
      rustandc_sum = unsafe {
          cbind::add_numbers(rustandc_sum, 1)
      }
    }
    let rustandc_time = i128::try_from(rustandc_timer.elapsed().as_nanos()).unwrap();

    println!("Rust loop time: {}ns Rust and C loop time: {}ns", rust_time, rustandc_time);
    assert_eq!((rustandc_time - rust_time).abs() < 100, true);
  }
}