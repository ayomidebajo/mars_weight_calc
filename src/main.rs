use std::io;

fn main() {
    println!("Enter your weight (kg)");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();
  
    let mars_weight = calculater_weight_on_mars(weight);
    println!("Weight on mars: {} g",  mars_weight);
   
}


fn calculater_weight_on_mars(weight: f32) -> f32  {
( weight / 9.81 ) * 3.711
}


