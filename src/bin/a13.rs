fn main()
{
  let vector = vec![10, 20, 30, 40];

  for number in &vector 
  {
    match number 
    {
      30 => println!("thirty"),
      _ => println!("{:?}", number)
    }
  }

  println!("Numbers of elements: {:?}", vector.len());
}