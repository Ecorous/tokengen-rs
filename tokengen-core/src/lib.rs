use base64::{Engine as _, engine::general_purpose};
use rand::Rng;

/**
 * If `length` is `None`, 32 will be used
  ```rs
  use tokengen_core::generate;

  fn main() {
    println!(generate(32)) // or use generate(None)
  }
 */
pub fn generate(length: Option<i32>) -> String {
    let mut clength = 0;
    let mut a = 32;
    if let Some(x) = length {
        a = x
    }
    let mut encoded = String::from("");
    while clength < a  {
        let mut rng = rand::thread_rng();
        let out: i32 = rng.gen();
        let nencoded= general_purpose::STANDARD_NO_PAD.encode(out.to_be_bytes());
        encoded += &nencoded;
        clength += nencoded.len() as i32;
        encoded.shrink_to(a as usize);
    }
    encoded.chars().take(a as usize).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result_32 = generate(None).len();
        let result_48 = generate(Some(48)).len();
        let result_64 = generate(Some(64)).len();
        let result_80 = generate(Some(80)).len();
        let result_96 = generate(Some(96)).len();
        assert_eq!(result_32, 32);
        assert_eq!(result_48, 48);
        assert_eq!(result_64, 64);
        assert_eq!(result_80, 80);
        assert_eq!(result_96, 96)
    }
}
