
enum Color
{
  Black,
  White
}

impl Color 
{
  fn print(self) -> String
  {
    match self 
    {
      Color::Black => String::from("black"),
      Color::White => String::from("white")    
    }
  }
}

struct Person
{
  name: String,
  color: Color,
  age: u32 
}

fn main()
{
  let student = vec!
  [ 
    Person 
    {
      name: "Luis".to_owned(),
      color: Color::Black,
      age: 23
    },
    Person
    {
      name: "Yenny".to_owned(),
      color: Color::Black,
      age: 23
    },
    Person
    {
      name: "Danny".to_owned(),
      color: Color::White,
      age: 26
    }
  ];

    for value in student 
    {
      println!("The name is: {0}", value.name);    
      println!("The age is: {0}", value.age);    
      println!("The color is: {0}\n", value.color.print())
    }

}