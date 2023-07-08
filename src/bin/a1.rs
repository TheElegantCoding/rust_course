fn display_first_name() -> &'static str
{
  return "Luis"
}

fn display_last_name() -> &'static str
{
  return "Monsalve"
}

fn main() 
{
  println!("{0} {1}", display_first_name(), display_last_name());
}
