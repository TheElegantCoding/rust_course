enum Color
{
  Red,
  Black,
  Blue
}

impl Color
{
  fn print(&self)
  {
    match self 
    {
      Color::Black => println!("color: black"),    
      Color::Red=> println!("color: red"), 
      Color::Blue=> println!("color: blue")    
    }
  }
}

struct Box 
{
  dimension: f32,
  weight: f32,
  color: Color,    
}

impl Box 
{
  fn new(dimension: f32, weight: f32, color: Color) -> Self
  {
    return Box 
    {
      dimension,
      weight,
      color
    }
  }

  fn display_details(&self)
  {
    println!("dimension: {0}", self.dimension);
    println!("weight: {0}", self.weight);
    self.color.print();
  }
}

fn main()
{
  let big_box = Box::new(20.0, 32.0, Color::Black);

  big_box.display_details();
}