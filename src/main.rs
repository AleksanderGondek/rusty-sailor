extern crate rusty_sailor;

fn main() {
  let settings = rusty_sailor::config::Settings::new();

  println!("Hello, world!");
  match settings {
    Ok(x) => {
      println!("Debug: {}", x.debug);
      println!("Pki.test: {}", x.pki.test);
    },
    Err(_) => {},
  }
}
