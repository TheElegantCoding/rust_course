enum Color
{
  Red,
  Blue,
  White,
  Black
}

fn main()
{
  let color = Color::Black;

  match color 
  {
    Color::Red => print!("The colors is red"),     
    Color::Blue => print!("The colors is blue"),     
    Color::White => print!("The colors is white"),     
    Color::Black => println!("The color is black"),
  }
}