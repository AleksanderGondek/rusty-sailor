extern crate rusty_sailor;

fn main() {
  let settings = rusty_sailor::config::Settings::new();

  println!("Hello, world!");
  match settings {
    Ok(x) => {
      println!("Debug: {}", x.debug);
      println!("Pki.test: {}", x.pki.test);
      println!("Pki.ca.test_two: {}", x.pki.ca.test_two);
    },
    Err(_) => {},
  }
}
