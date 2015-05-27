#[derive(Debug)]
struct Structure(i32);

fn main() {
  println!("{:?} months in a year", 12);
  println!("{1:?} {0:?} is the {actor:?} name",
    "Slater",
    "Christian",
    actor="actor's");

  println!("Now {:?} will print!",
    Structure(3));
}
