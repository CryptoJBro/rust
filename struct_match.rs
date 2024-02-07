fn main() {
   let coke = Drink {
    flavor: Flavor::Cola,
    ounce: 12.0,
   };
   print_drink(coke);
 
   let strawberry = Drink {
    flavor: Flavor::Strawberry,
    ounce: 20.0,
   };
   print_drink(strawberry);
 }

enum Flavor {
    Cola,
    Strawberry,
 }
 
 struct Drink {
    flavor: Flavor,
    ounce: f64,
 }
 
 fn print_drink(drink: Drink) {
    match drink.flavor {
       Flavor::Cola => println!("Cola!"),
       Flavor::Strawberry => println!("Strawberry"),
    }
    
    println!("oz: {:?}", drink.ounce);
 }
