fn coordinate () -> (i32, i32, i32)
{
  return (1, 5, 8)
}

fn main ()
{
  let (x, y, z) = coordinate();

  if y > 5
  {
    println!("y is grater than 5")
  } 
  else if y < 5 
  {
    println!("y us lees than 5")    
  }
  else
  {
    println!("y is 5")    
  }
}