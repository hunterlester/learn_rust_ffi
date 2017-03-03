extern crate rand;
use rand::{OsRng, Rng};

#[no_mangle]
pub extern "C" fn roll() -> u8 {

  let choices: [u8; 6] = [1, 2, 3, 4, 5, 6];

  let mut rng = match OsRng::new() {
    Ok(t) => t,
    Err(e) => panic!("FAIL!, {}", e),
  };

  let rand_num: u8 = *rng.choose(&choices).unwrap();

  rand_num

}
