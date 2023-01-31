fn main() {
    let first_name = String::from("Greg").to_owned() + "Osin";
    let name = &first_name;
    println!("Hello, {}!", name);
    println!("Hello, {}!", first_name);
}

fn main() {
    let mut first_name = String::from("Greg");
    first_name.push_str(" Osin");

    println!("Hello, {}!", first_name);
}

fn main() {
    let first = "∞";
    println!("{} {}",first.len(),first.chars().count())
  
  }

// Lesson #26
fn main() {

}

#[cfg(test)]
mod tests {
  use crate::main;
  #[test]
  fn main_returns_24() {
    assert_eq!(main(), 24);
  }
}

// Lesson #27
fn main() -> f32 {
    24.5
  }
  
  #[cfg(test)]
  mod tests {
    use crate::main;
    #[test]
    #[should_panic]
    fn main_panics_with_i() {
      assert_eq!(main() as usize as f32, main() as f32);
    }
    #[test]
    fn main_returns_f() {
      assert_eq!(main() as f32, 24.5);
    }
  }

  
// Lesson #28
fn main() -> f32 {
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
      assert_eq!(main() as usize as f32, main() as f32);
    }
    #[test]
    fn main_returns_f() {
      assert_eq!(main() as f32, 24.5);
    }
  
    #[test]
    fn output_expects_four_args() {
      let out = output(-10, '+', 10, 0);
      assert_eq!(out, ());
    }
  }
  