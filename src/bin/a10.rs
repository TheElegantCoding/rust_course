fn main ()
{
  let number = 500;
  let validation = number > 100;

  if validation
  {
    println!("It's big");
    return;
  }

  println!("Its's small");
}