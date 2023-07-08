enum Flavor
{
  Orange, 
  Chocolate
}

struct Drink
{
  flavor: Flavor,
  ounce: f32
}

fn show_drink(flavor: Flavor, ounce: f32)
{
  let drink = Drink 
  {
    flavor: flavor,
    ounce: ounce
  };

  match drink.flavor 
  {
    Flavor::Chocolate => println!("The Drink flavor is chocolate"),
    Flavor::Orange => println!("The Drink flavor is orange"),
  }

  println!("The Drink ounce is: {:?}", drink.ounce);
  println!("Delicious !!");
}


fn main()
{
  show_drink(Flavor::Chocolate, 1.5);
}