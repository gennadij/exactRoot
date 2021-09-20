use std::env;

mod exact_root;

fn main() {
  let args: Vec<String> = env::args().collect();

  let result: u64 = match &args[1].parse() {
    Ok(rad) => exact_root::berechne_exacte_wurzel(*rad),
    Err(e) => {
      println!("Error : {}", e); 
      0
    }
  };

  println!("Exakte Wurzel ist {}", result);
}
