//There are many types of number, in Rust:
//  - Unsigned Integers: u8, u16, u32, u64, usize, u128
//  - Signed Integer: i8, i16, i32, i64, isize, i128
//  -  Float: f32, f64

fn main() {
    return_value();
    return_float();
    
    let first_number = 1;
    let operator = '+';
    let second_number = 3;
    
    println!("{}", output(first_number, operator, second_number, 4));
}

// Lesson #26
fn return_value()-> usize {
    24
}

// Lesson #27
fn return_float()-> f32 {
  24.5
}

// lesson #28
fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String{
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

//lesson 35
fn operate(operator: char, first_number: i32, second_number: i32) -> i32{
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '*' => first_number * second_number,
    '/' => first_number / second_number,
    _ => panic!("Invalid operator used")
  }
}

/* function based on else if 
//lesson 34
fn operate(operator: char, first_number: i32, second_number: i32) -> i32{
  if operator == '+'{
    first_number + second_number
  }
  else if operator == '-' {
    first_number - second_number
  }
  else if operator == '*' {
    first_number * second_number
  }
  else if operator == '/' {
    first_number / second_number
  }
  else {
    panic!("Invalid operator used")
  }
}
*/
  //added tests for output function
#[cfg(test)]
  mod tests {
    use crate::main;
    use crate::output;
    use crate::operate;
    #[test]
    fn main_returns_empty_tuple() {
      assert_eq!(main(), ());
    }
  
    #[test]
    fn output_expects_four_args() {
      let out = output(-10, '+', 10, 0);
      assert_eq!(out, String::from("-10 + 10 = 0"));
    }
  
    #[test]
    fn operate_handles_addition() {
      let op = operate('+', -5, 200);
      assert_eq!(op, 195);
    }
    #[test]
    fn operate_handles_subtraction() {
      let op = operate('-', -12, -12);
      assert_eq!(op, 0);
    }
    #[test]
    fn operate_handles_division() {
      let op = operate('/', -12, -1);
      assert_eq!(op, 12);
    }
    #[test]
    fn operate_handles_multiplication() {
      let op = operate('*', -12, -2);
      assert_eq!(op, 24);
    }
    #[test]
    #[should_panic]
    fn operate_panics_on_invalid_op() {
      operate('a', 1, 1);
    }
  }

/* old tests for functions
//main tests for integer
#[cfg(test)]
mod tests {
  use crate::return_value;
  #[test]
  fn main_returns_24() {
    assert_eq!(return_value() as i32, 24);
  }
}
//added test for float
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
 */