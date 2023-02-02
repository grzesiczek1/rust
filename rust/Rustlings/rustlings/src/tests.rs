//There are many types of number, in Rust:
//  - Unsigned Integers: u8, u16, u32, u64, usize, u128
//  - Signed Integer: i8, i16, i32, i64, isize, i128
//  -  Float: f32, f64

fn main() {
    return_value();
    return_float();
}

// Lesson #26
fn return_value()-> usize {
    24
}

// Lesson #27
fn return_float()-> f32 {
  24.5
}

#[cfg(test)]
mod tests {
  use crate::return_value;
  #[test]
  fn main_returns_24() {
    assert_eq!(return_value() as i32, 24);
  }
}
#[cfg(test)]
  mod tests_float {
    use crate::return_float;
    #[test]
    #[should_panic]
    fn main_panics_with_i() {
      assert_eq!(return_float() as usize as f32, return_float() as f32);
    }
    #[test]
    fn main_returns_f() {
      assert_eq!(return_float() as f32, 24.5);
    }
  }


  /*
// Lesson #28
fn main5() -> f32 {
    24.5
  }
  
  fn output(_first: i32, _second: char, _third: i32, _fourth: i32){
  
  }
  
  
  #[cfg(test)]
  mod tests {
    use crate::main;
    use crate::output;
    #[test]
    #[should_panic]
    fn main_panics_with_i() {
      assert_eq!(main5() as usize as f32, main5() as f32);
    }
    #[test]
    fn main_returns_f() {
      assert_eq!(main5() as f32, 24.5);
    }
  
    #[test]
    fn output_expects_four_args() {
      let out = output(-10, '+', 10, 0);
      assert_eq!(out, ());
    }
  }

  */