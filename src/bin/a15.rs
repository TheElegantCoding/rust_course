enum Ticket 
{
  BackStage(f64, String),
  Vip(f64, String),
  Standard(f64)
}

fn main()
{
  let event_ticket = vec![
    Ticket::BackStage(310.0, "Luis".to_owned()), Ticket::Vip(500.0, "Danny".to_owned()), Ticket::Standard(70.0)
  ];

  for ticket in event_ticket 
  {
    match ticket 
    {
      Ticket::BackStage(price, holder) => println!("Holder: {:?}, Price: {:?}", holder, price),
      Ticket::Vip(price, holder) => println!("Holder: {:?}, Price: {:?}", holder, price),
      Ticket::Standard(price) => println!("Price: {:?}",price)
    }
  }
}