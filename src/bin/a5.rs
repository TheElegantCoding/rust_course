fn main()
{
  let mut number = 0;

  loop
  {
    number = number + 1;
    println!("The number is: {0}", number);

    if number == 4
    {
      break;
    }
  }
}