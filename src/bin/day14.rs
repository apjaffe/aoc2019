use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Reaction {
  reactants: HashMap<String, i64>,
  product: String,
  product_quantity: i64
}

fn make_products(product: &str, quantity: i64, reactions: &HashMap<String, Reaction>, spare: &mut HashMap<String, i64>) -> i64 {
  //println!("We need {} of {}, we have {:?}", quantity, product, spare);
  if product == "ORE" {
    return quantity
  }
  let q = match spare.get_mut(product) {
    Some(spareq) => {
      if *spareq >= quantity {
        *spareq = *spareq - quantity;
        return 0;
      } else {
        let newq = quantity - *spareq;
        *spareq = 0;
        newq
      }
    },
    None => quantity
  };
  let mut sum = 0;
  let reaction = reactions.get(product).unwrap();
  let rem = q % reaction.product_quantity;
  let mult = q / reaction.product_quantity + (if rem == 0 { 0 } else { 1 });
  //println!("Apply reaction {} times {} to get {} from {}", product, mult, q, reaction.product_quantity);
  for (reactant, reactantq) in reaction.reactants.iter() {
    sum += make_products(reactant, reactantq * mult, reactions, spare);
  }
  spare.insert(product.to_string(), mult * reaction.product_quantity-q + spare.get(product).unwrap_or(&0));
  sum
}

fn main() {
    let stdin = io::stdin();
    let mut reactions = HashMap::new();
    for line in stdin.lock().lines() {
      let unwrapped = line.unwrap();
      let row = unwrapped.split(" => ").collect::<Vec<&str>>();
      let row0 = row[0].to_string();
      let row1 = row[1].to_string();
      let reactants_str = row0.split(", ").collect::<Vec<&str>>();
      let product_str = row1.split(" ").collect::<Vec<&str>>();
      let product_quantity = product_str[0].parse::<i64>().unwrap();
      let product = product_str[1].to_string();
      let mut reactants = HashMap::new();
      for reactant_str in reactants_str {
        let reactant_split = reactant_str.split(" ").collect::<Vec<&str>>();
        let reactant_quantity = reactant_split[0].parse::<i64>().unwrap();
        let reactant = reactant_split[1].to_string();
        reactants.insert(reactant, reactant_quantity);
      }
      reactions.insert(product.to_string(), Reaction{
        reactants: reactants,
        product: product,
        product_quantity: product_quantity
      });
    }
    println!("{:?}", reactions);
    let mut spare = HashMap::new();
    let res = make_products("FUEL", 1, &reactions, &mut spare);
    println!("{:?}", res);
    let mut low = 1;
    let mut high = 1000000000000;
    let ore = 1000000000000;
    while low < high - 1 {
      let guess = (low+high)/2;
      let mut spare = HashMap::new();
      let ore_needed = make_products("FUEL", guess, &reactions, &mut spare);
      if ore_needed > ore {
        high = guess;
      } else {
        low = guess;
      }
    }
    println!("{}", low);
}
