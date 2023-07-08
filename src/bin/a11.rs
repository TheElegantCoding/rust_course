struct Grocery
{
  id: u32,
  quantity: i32
}

fn display_id(id: u32)
{
  println!("The id is: {0}", id);
} 

fn display_quantity(quantity: i32)
{
  println!("The quantity is: {0}", quantity);
} 

fn main()
{
  let onion = Grocery 
  {
    id: 1,
    quantity: 10
  };

  display_id(onion.id);
  display_quantity(onion.quantity);
}