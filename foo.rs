use std::fs::File;
use std::io::Write;

fn write(name: std::string::String) {
    let mut f = File::create(name).unwrap();
    writeln!(&mut f, "This is a test").unwrap();
}

fn main() {
  let an_integer = 1u32;
  let a_boolean = true;
  let unit = ();

  // copy `an_integer` into `copied_integer`
  let copied_integer = an_integer;

  println!("An integer: {:?}", copied_integer);
  println!("A boolean: {:?}", a_boolean);
  println!("Meet the unit value: {:?}", unit);

  println!("Hello rust world!");

  write(String::from("test.txt"));
  println!("Wrote file");

  std::fs::copy("test.txt", "test-2.txt").unwrap();
  println!("Copied file");
}